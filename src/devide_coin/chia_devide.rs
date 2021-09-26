use bigdecimal::{FromPrimitive, ToPrimitive};
use diesel::Connection;

use crate::db::MysqlPooledConnection;
use crate::models::chia_block_histories_model::*;
use crate::models::coin_pool_model::get_spaces_count;
use crate::models::coin_pool_ratio_model::get_pool_ratio;
use std::process::Command;

/* 这里只有当爆块程序放在和PHP同一台服务器上时才可以使用 */
pub fn php_devide_chia(new_block: &AddChiaBlockHistories, connection: &MysqlPooledConnection) {
    let coin_id = &new_block.coin_id;

    let two_arg = format!("crontab/autochia/{}", coin_id);
    let one_arg = "/www/wwwroot/bzz.mingyou168.cn/public/index.php";

    println!("执行的分币命令： php {} {}", one_arg, two_arg);

    /* 执行命令，使用PHP去分币 */
    let mut output = Command::new("php")
        .arg(one_arg)
        .arg(two_arg)
        .output()
        .expect("执行命令失败");

    println!("chia执行自动分币命令结果：{:#?}", output);
}

/* Rust分币(记得启用事务) */
pub fn devide_chia(new_block: &AddChiaBlockHistories, connection: &MysqlPooledConnection) -> bool {
    use crate::models::coin_pool_allot_model::*;
    use chrono::prelude::{Local, NaiveDate, NaiveDateTime};

    let coin_id = &new_block.coin_id;

    let check = check_allot(coin_id, connection);
    if !check {
        return false;
    }

    let spaces_count = get_spaces_count(new_block.pool_id, connection);
    if spaces_count <= 0 {
        return false; //矿池总算力小于0不能分币
    }

    let ratio_data = get_pool_ratio(new_block.pool_id, connection);
    if ratio_data.is_empty() {
        return false; //矿池分配比例为空
    }

    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now();
    let dft = now.format(fmt);
    let str_date = dft.to_string();
    println!("当前时间：{}", str_date);
    let now_date_time = NaiveDateTime::parse_from_str(str_date.as_str(), fmt).unwrap();
    let now_date = NaiveDate::parse_from_str(str_date.as_str(), "%Y-%m-%d").unwrap();

    /* 开启事务 */
    let _temp = connection.transaction::<_, diesel::result::Error, _>(|| {
        /* 这里写包含的事务处理 */
        for ratio in ratio_data {
            let amount = new_block.amount.unwrap() as f64;
            let amount = amount / 1000000000000.; //得到的币数0.25个,12位
            let ratio_temp = ratio.ratio.clone().unwrap().to_f64().unwrap();

            let temp = ratio.spaces as f64 / spaces_count as f64;
            let temp = amount * temp;
            let temp_number = temp * ratio_temp / 100.;
            if temp_number <= 0.0 {
                println!(
                    "此次分不到币，可能是比例或者系数太小:{};系数：{}",
                    temp_number, ratio_temp
                );
                continue;
            }

            let bigdecimal_number = bigdecimal::BigDecimal::from_f64(temp_number).unwrap();

            let new_allot = AddCoinPoolAllot {
                ratio_id: ratio.id,
                daily: now_date, //当天的时间  y-m-d
                pool_id: ratio.pool_id.unwrap(),
                pool_name: ratio.pool_name,
                pool_type: ratio.pool_type.unwrap(),
                pool_wallet_address: new_block.wallet_address.clone(),
                spaces_count: spaces_count,
                customer_id: ratio.customer_id,
                username: ratio.username,
                customer_wallet_address: ratio.customer_wallet_address,
                spaces: ratio.spaces,
                ratio: ratio.ratio.unwrap(),
                coin_number: bigdecimal_number,
                amount: bigdecimal::BigDecimal::from_f64(amount).unwrap(),
                block_height: new_block.block_height,
                block_time: new_block.timestamp,
                status: 0,
                admin_id: None,
                create_time: Some(now_date_time),
                last_time: Some(now_date_time),
            };
            let insert_row = new_allot.insert(connection);
            if insert_row < 1 {
                println!("插入分币记录出错");
            }
        }

        /* 更改爆块为已分币:chia是通过coin_id来区分每个块不同 */
        let update_row = set_status(&new_block.coin_id, 1, now_date_time, connection);
        if update_row < 1 {
            println!("更新CHIA块为已分币出错");
        }
        Ok(())
    });

    return true;
}
