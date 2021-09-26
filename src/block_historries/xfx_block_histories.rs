use crate::db::MysqlPooledConnection;
use crate::json_value::xfx_block_hitories_value::{XFXWallet, XFXDATA};
use crate::models::coin_pool_groover_model::get_groover;
use crate::models::xfx_block_histories_model::*;
use bigdecimal::BigDecimal;
use chrono::prelude::Local;
use chrono::TimeZone;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

pub async fn xfx_block_to_mysql(
    path: PathBuf,
    connection: &MysqlPooledConnection,
    message: &mut String,
) {
    let json_path = "path.clone()".to_string();
    let file = File::open(&path).expect("打开JSON文件出错");
    let reader = BufReader::new(&file);

    // let data: XFXDATA = serde_json::from_reader(reader).expect("解析XFX的JSON文件出错");
    let data: XFXDATA = serde_json::from_reader(reader).unwrap_or_else(|op| {
        panic!("数据查询失败: {:#?}", op);
    });
    let xfx_wallets = data.data;
    for xfx_wallet in xfx_wallets {
        // one_histories(&xfx_wallet, connection, message);
        let send_array = one_block_histories(&xfx_wallet, connection);

        if send_array.is_empty() {
            continue;
        }

        /* 添加要发送的消息 */
        for (_, send_sms) in send_array {
            let temp = &format!(
                "\n \r\n  矿池：{}（{}）爆块了，\n区块高度{},\n时间：{}，\n获得：{}个XFX",
                send_sms.pool_name,
                send_sms.groover_name,
                send_sms.block_height,
                send_sms.block_time,
                send_sms.amount
            );
            message.push_str(temp);

            let temp = &format!("\n此矿工今日爆块：{}个", send_sms.groover_count);
            message.push_str(temp);
        }
    }
}

/* 定义消息发送处理结构体 */
#[derive(Debug, Clone)]
pub struct SendSms {
    pub block_height: i64,    //区块高度
    pub block_time: String,   //爆块时间
    pub pool_name: String,    //矿池名
    pub groover_name: String, //矿工名
    pub groover_count: i64,   //此矿工今日爆块
    pub amount: String,       //获得XFX的个数
}

pub fn one_block_histories(
    xfx_wallet: &XFXWallet,
    connection: &MysqlPooledConnection,
) -> HashMap<i64, SendSms> {
    let wallet_address = &xfx_wallet.address;

    let groover_data = get_groover(wallet_address, connection);

    let groover_id = groover_data.0;
    let pool_id = groover_data.1;
    let pool_name = groover_data.2.unwrap();
    let pool_type = groover_data.3.unwrap();
    let pool_order = groover_data.4.unwrap();
    let groover_name = groover_data.5.unwrap();
    // let wallet_id = groover_data.6.expect("钱包ID出错");
    let wallet_id = groover_data.6.unwrap_or_else(|| {
        let temp = String::new();
        return temp;
    });

    let records = &xfx_wallet.record;
    // let mut send_array: Vec<SendSms> = Vec::new();
    let mut send_array: HashMap<i64, SendSms> = HashMap::new();

    for record in records {
        let coin_hash = &record.coin;
        //查找此区块是否已记录过，已记录过则跳过
        let block_id = get_tx_id(coin_hash, &connection);
        if block_id != 0 {
            println!("路过处理");
            continue;
        }

        let block_height = &record.blockHeight;
        let block_height = block_height.parse::<i64>().expect("字符串转i64类型出错");
        let height_spent = &record.heightSpent;
        let block_time = record.time;
        // let block_time = &record.time;
        // let block_time = block_time.parse::<i64>().expect("字符串转i64类型出错");
        let time_str = &record.time_str;
        let amount = &record.amount;
        let amount_big = BigDecimal::from_str(&amount.to_owned()).expect("msg");
        let amount_text = &record.amountText;
        let amount_mojo = &record.amountMojo;

        let new_xfx = AddXFXBlockHistories {
            tx_id: coin_hash.clone(),
            block_height: block_height,
            height_spent: Some(height_spent.clone()),
            block_time: block_time,
            time_str: Some(time_str.clone()),
            block_type: None,
            amount: Some(amount_big),
            amount_text: Some(amount_text.clone()),
            amount_mojo: Some(amount_mojo.clone()),
            pool_id: pool_id,
            pool_name: Some(pool_name.clone()),
            pool_type: Some(pool_type.clone()),
            groover_id: groover_id,
            pool_order: Some(pool_order),
            groover: Some(groover_name.clone()),
            wallet_id: Some(wallet_id.clone()),
            wallet_address: Some(wallet_address.clone()),
            status: 0, //未分币
            modify: None,
            create_time: None,
        };

        let row = new_xfx.insert(connection);
        if row > 0 {
            println!("插入新的爆块成功，发送钉钉消息");

            match send_array.get(&block_height) {
                Some(old_send_sms) => {
                    let groover_count = get_groover_day_count(groover_id, connection);
                    let amount = amount.parse::<f64>().expect("字符串转f64类型出错");
                    let old_amout = old_send_sms
                        .amount
                        .parse::<f64>()
                        .expect("字符串转f64类型出错");

                    send_array.insert(
                        block_height,
                        SendSms {
                            block_height: block_height,
                            groover_count: groover_count / 2, //此矿工今日爆块数除以2为正确数
                            pool_name: pool_name.clone(),
                            groover_name: groover_name.clone(),
                            block_time: old_send_sms.block_time.clone(),
                            amount: format!("{}+{}={}", old_amout, amount, old_amout + amount),
                        },
                    );
                }
                None => {
                    let block_time = Local.timestamp(block_time, 0);
                    send_array.insert(
                        block_height,
                        SendSms {
                            block_height: block_height,
                            groover_count: 0, //此矿工今日爆块数
                            pool_name: pool_name.clone(),
                            groover_name: groover_name.clone(),
                            block_time: block_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                            amount: amount.clone(),
                        },
                    );
                }
            }
        }
    }
    send_array
}

pub fn _one_histories(
    xfx_wallet: &XFXWallet,
    connection: &MysqlPooledConnection,
    message: &mut String,
) {
    let wallet_address = &xfx_wallet.address;

    let groover_data = get_groover(wallet_address, connection);

    let groover_id = groover_data.0;
    let pool_id = groover_data.1;
    let pool_name = groover_data.2.unwrap();
    let pool_type = groover_data.3.unwrap();
    let pool_order = groover_data.4.unwrap();
    let groover_name = groover_data.5.unwrap();
    // let wallet_id = groover_data.6.expect("钱包ID出错");
    let wallet_id = groover_data.6.unwrap_or_else(|| {
        let temp = String::new();
        return temp;
    });

    let records = &xfx_wallet.record;
    for record in records {
        let coin_hash = &record.coin;
        //查找此区块是否已记录过，已记录过则跳过
        let block_id = get_tx_id(coin_hash, &connection);
        if block_id != 0 {
            println!("路过处理");
            continue;
        }

        let block_height = &record.blockHeight;
        let block_height = block_height.parse::<i64>().expect("字符串转i64类型出错");
        let height_spent = &record.heightSpent;
        let block_time = record.time;
        // let block_time = &record.time;
        // let block_time = block_time.parse::<i64>().expect("字符串转i64类型出错");
        let time_str = &record.time_str;
        let amount = &record.amount;
        let amount_big = BigDecimal::from_str(&amount.to_owned()).expect("msg");
        let amount_text = &record.amountText;
        let amount_mojo = &record.amountMojo;

        let new_xfx = AddXFXBlockHistories {
            tx_id: coin_hash.clone(),
            block_height: block_height,
            height_spent: Some(height_spent.clone()),
            block_time: block_time,
            time_str: Some(time_str.clone()),
            block_type: None,
            amount: Some(amount_big),
            amount_text: Some(amount_text.clone()),
            amount_mojo: Some(amount_mojo.clone()),
            pool_id: pool_id,
            pool_name: Some(pool_name.clone()),
            pool_type: Some(pool_type.clone()),
            groover_id: groover_id,
            pool_order: Some(pool_order),
            groover: Some(groover_name.clone()),
            wallet_id: Some(wallet_id.clone()),
            wallet_address: Some(wallet_address.clone()),
            status: 0, //未分币
            modify: None,
            create_time: None,
        };

        let row = new_xfx.insert(connection);
        if row > 0 {
            println!("插入新的爆块成功，发送钉钉消息");
            let block_time = Local.timestamp(block_time, 0);
            let temp = &format!(
                "\n \r\n  矿池：{}（{}）爆块了，\n区块高度{},\n时间：{}，\n获得：{}个XFX",
                pool_name,
                groover_name,
                block_height,
                block_time.format("%Y-%m-%d %H:%M:%S"),
                amount
            );
            message.push_str(temp);

            /* 统计此矿工今日 */
            let groover_count = get_groover_day_count(groover_id, connection);
            let temp = &format!("\n此矿工今日爆块：{}个", groover_count);
            message.push_str(temp);
        }
    }
}
