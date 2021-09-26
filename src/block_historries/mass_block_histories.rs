use crate::common::get_ssh_session;
use crate::db::MysqlPooledConnection;
use crate::devide_coin::mass_devide::*;
use crate::dingtalk::send;
use crate::json_value::mass_block_histories_value::*;
use crate::models::mass_block_histories_model::*;
use bigdecimal::BigDecimal;
use chrono::TimeZone;

use chrono::prelude::{Local, NaiveDateTime};

use std::io::prelude::*;
use std::str::FromStr;

pub async fn command_find_block_histories(
    pool_id: i64,
    pool_name: &String,
    groover_id: i64,
    groover_name: &String,
    wallet_id: &String,
    wallet: &String,
    is_allot: i8,
    connection: &MysqlPooledConnection,
) {
    // println!("执行取得爆块信息");
    let ip = "10.8.0.165:22"; //可登录的钱包节点IP
    let session = get_ssh_session(ip); //取得登录SESSION

    /* 这里执行切换钱包ID */
    let mut channel = session.channel_session().unwrap();
    let mut output = String::new();
    let command = format!("/root/mass-wallet/masswalletcli usewallet {}", wallet_id);
    channel.exec(&command.to_owned()).unwrap();
    channel.read_to_string(&mut output).unwrap();

    /* 取得1000条爆块信息，一次最多只能取1000,  稳定后缩短为：10条 */
    let command = format!(
        "/root/mass-wallet/masswalletcli listtransactions count=10 address={}",
        wallet
    ); //切换钱包ID命令
    let mut channel = session.channel_session().unwrap();
    channel.exec(&command.to_owned()).expect("执行命令出错");
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();

    let mass_block_histories: MassBlockHistories =
        serde_json::from_str(&output.to_owned()).expect("解析JSON出错2222");

    let mut message = String::new(); //定义要发送的消息提醒

    for histories in mass_block_histories.histories {
        let from_addresses = &histories.fromAddresses;
        let from_addresses = from_addresses[0].clone();
        if !from_addresses.eq("COINBASE") {
            continue; //不是爆块则直接跳过，进入下一次循环处理
        }

        // println!("进入:处理爆块");

        let tx_id = &histories.txId; //交易ID

        let old_block = get_tx_id(tx_id, &connection);
        let id = old_block.0;
        // 爆块已记录过，并且矿池ID也记录了，就直接跳过 (id=8,pool_id=0)
        if !id.eq(&0) && !old_block.1.eq(&0) {
            // println!("进入:跳过处理");
            continue;
        }

        deal_histories(
            pool_id,
            pool_name,
            groover_id,
            groover_name,
            wallet,
            is_allot,
            &histories,
            &session,
            connection,
            &mut message,
        );
    }

    // println!("发送钉钉消息:{}", message);
    if !message.is_empty() {
        let groover_count = get_groover_day_count(groover_id, &connection);
        message += &format!("\n此矿工今日爆块：{}个", groover_count);

        let today_count = get_day_count(pool_id, &connection);
        message += &format!("\n此矿池今日爆块：{}个", today_count);
        let count = get_pool_count(pool_id, &connection);
        message += &format!("\n此矿池总爆块：{}个", count);

        let today_count = get_day_count_all(&connection);
        message += &format!("\n今日总爆块：{}个", today_count);
        // let count = get_all_count(&connection);
        // message += &format!("\n总爆块：{}个", count);
        let temp = send(message).await;
        println!("发送消息结果：{:?}", temp);
    }
}

pub fn deal_histories(
    pool_id: i64,
    pool_name: &String,
    groover_id: i64,
    groover_name: &String,
    wallet: &String,
    is_allot: i8,
    histories: &Histories,
    session: &ssh2::Session,
    connection: &MysqlPooledConnection,
    message: &mut String,
) {
    let from_addresses = &histories.fromAddresses;
    let from_addresses = &from_addresses[0];
    let tx_id = &histories.txId; //交易ID

    let old_block = get_tx_id(&tx_id, connection);
    let id = old_block.0;
    // println!("进入:真正处理程序");
    let block_height = &histories.blockHeight; //区块高度
    let outputs = &histories.outputs;
    let amount = find_amount(&outputs, &wallet);

    /* 通过块高度(blockHeight)查详情，取得爆块时间 */
    // let command = "/root/mass-wallet/masswalletcli getblockbyheight 1437687"; //通过区块高度取得区块详情
    let command = format!(
        "/root/mass-wallet/masswalletcli getblockbyheight {}",
        block_height
    );
    let mut channel = session.channel_session().unwrap();
    channel.exec(&command.to_owned()).expect("执行命令出错");
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();

    let data: BlockHeight =
        serde_json::from_str(&output.to_owned()).expect("出错了，解析区块高度json文件 ");

    let block_time = &data.time;

    let block_height = block_height.parse::<i64>().expect("字符串转i64类型出错");
    let block_time = block_time.parse::<i64>().expect("字符串转i64类型出错");
    // let amount = BigDecimal::from_str(&amount.to_owned()).expect("msg");

    if id.eq(&0) {
        // println!("先看下是否执行自动分币，再插入爆块信息,并且要发送消息到钉钉提醒");

        let mut status: i8 = 0; //是否分币（0未分币，1已分币）

        let mut new_block = AddMassBlockHistories {
            tx_id: tx_id.clone(),
            block_height: block_height,
            block_time: block_time,
            amount: BigDecimal::from_str(&amount.to_owned()).expect("msg"),
            from_addresses: "COINBASE".to_string(),
            wallet_address: wallet.to_string(),
            pool_id: Some(pool_id),
            pool_name: Some(pool_name.clone()),
            pool_type: Some("mass".to_string()),
            pool_address: None,
            groover_id: Some(groover_id),
            groover_name: Some(groover_name.clone()),
            status: status,
            modify: None,
        };
        new_block.insert(&connection); //插入数据

        //格式化时间：
        let block_time = Local.timestamp(block_time, 0);
        let temp = &format!(
            "\n \r\n {}-矿池（矿工{}）爆块了，\n区块高度{},\n时间：{}，\n获得：{}个mass",
            pool_name,
            groover_name,
            block_height,
            block_time.format("%Y-%m-%d %H:%M:%S"),
            amount
        );

        // message += temp.clone();
        message.push_str(temp);

        /* 爆块已插入，再执行命令去处理是否分币 */
        if is_allot.eq(&1) {
            println!("先去取得矿池的分币比例（coin_pool_ratio），再按比例分配,分币");

            devide_mass(&mut new_block, &connection);

            status = 1;
            let temp = &format!("执行自动分币:{}", status);
            // message += temp.clone();
            // message.push_str(temp);
        }
        // println!("哈哈，有了：{}", message);
    } else {
        println!("更新矿池信息到爆块表");
        update_pool(id, (pool_id, &pool_name, "mass"), &connection);
    }
}

pub fn find_amount(outputs: &Vec<Output>, wallet: &String) -> String {
    let mut amount = String::new();
    for output in outputs {
        if output.address.eq(wallet) {
            amount = output.amount.clone();
            break;
        }
    }
    amount
}
