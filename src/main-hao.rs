use bigdecimal::BigDecimal;
use diesel::types::ops::Add;
use models::mass_block_histories_model::*;
use ssh2::Session;
use std::net::TcpStream;
use std::path::Path;
use crate::db::establish_connection;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

use serde::Deserialize;

mod common;
mod db;
mod json_value;
mod models;
mod schema;

#[macro_use]
extern crate diesel;

fn main() {
    println!("远程登录执行命令取得mass爆块信息");

    let mut insert_datas: Vec<&AddMassBlockHistories> = Vec::new();
    let connection = establish_connection();

    let pubkey = Path::new("/root/.ssh/id_rsa.pub");
    let privatekey = Path::new("/root/.ssh/id_rsa");
    let ip = "10.8.0.165:22";
    let alert = format!("远程连接矿机{}出错", ip);
    let tcp = TcpStream::connect(ip).expect(&alert);
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session
        .userauth_pubkey_file("root", Some(pubkey), privatekey, None)
        .expect(&alert);

    if !session.authenticated() {
        println!("登录矿机失败");
    }

    let mut channel = session.channel_session().unwrap();

    let mut output = String::new();

    // let command="cd /root/mass-wallet";  //要执行的命令 ,切换目录到可执行文件
    let wallet_id = "ac10h7jxr4avmguekde639xjvkw6ep0w44me3vrrae"; //钱包ID
    let wallet = "ms1qqszncc5tfk6a3vx0d3nuf3fgaxhje8h8m88eqw6f3nz4ycvexl2fq8tpjpq"; //钱包地址

    // let command ="/root/mass-wallet/masswalletcli usewallet ac10h7jxr4avmguekde639xjvkw6ep0w44me3vrrae"; //切换钱包ID命令
    let command = format!("/root/mass-wallet/masswalletcli usewallet {}", wallet_id);
    channel.exec(&command.to_owned()).unwrap();
    channel.read_to_string(&mut output).unwrap();
    // println!("切换钱包（钱包ID）命令执行结果:{:#?}", output);
    // {
    //     "chainId": "5433524b370b149007ba1d06225b5d8e53137a041869834cff5860b02bebc5c7",
    //     "walletId": "ac10h7jxr4avmguekde639xjvkw6ep0w44me3vrrae",
    //     "type": 1,
    //     "totalBalance": "205741.30985921",
    //     "externalKeyCount": 2
    //   }
    // let data: WalletData = serde_json::from_str(&output.to_owned()).expect("json解析出错");
    // println!("成功后的json:{:#?}", data);

    // let command="/root/mass-wallet/masswalletcli listtransactions count=3 address=ms1qqszncc5tfk6a3vx0d3nuf3fgaxhje8h8m88eqw6f3nz4ycvexl2fq8tpjpq"; //切换钱包ID命令
    let command = format!(
        "/root/mass-wallet/masswalletcli listtransactions count=1000 address={}",
        wallet
    ); //切换钱包ID命令
    let mut channel = session.channel_session().unwrap();
    channel.exec(&command.to_owned()).expect("执行命令出错");
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();
    // println!("执行的命令输出：{}", output);
    let mass_block_histories: MassBlockHistories =
        serde_json::from_str(&output.to_owned()).expect("解析JSON出错2222");
    // println!("luck：{:#?}", mass_block_histories);

    /* 开始循环处理：判断是否为爆块（fromAddresses = COINBASE 时是爆块了），再以区块高度blockHeight查询爆块时间，最后组合数据插入表mass_block_histories */
    for histories in mass_block_histories.histories {
        // let from_addresses=histories.fromAddresses.pop().unwrap();
        let from_addresses = histories.fromAddresses;
        let from_addresses = &from_addresses[0];
        if !from_addresses.eq("COINBASE") {
            continue; //不是爆块则直接跳过，进入下一次循环处理
        }

        let tx_id = histories.txId; //交易ID



        let block_height = histories.blockHeight; //区块高度
        let outputs = histories.outputs;
        let amount = find_amount(&outputs, wallet);

        /* 通过块高度(blockHeight)查详情，取得爆块时间 */
        // 命令：./masswalletcli getblockbyheight 1437687
        // let command = "/root/mass-wallet/masswalletcli getblockbyheight 1437687"; //通过区块高度取得区块详情
        let command = format!(
            "/root/mass-wallet/masswalletcli getblockbyheight {}",
            block_height
        );
        let mut channel = session.channel_session().unwrap();
        channel.exec(&command.to_owned()).expect("执行命令出错");

        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();
        //再定义 结构体解析区块
        let data: BlockHeight =
            serde_json::from_str(&output.to_owned()).expect("出错了，解析区块高度json文件 ");

        let block_time = &data.time;
        // println!("区块高度：{:#?}", data);

        // let update_timestamp = &data.update_timestamp.parse::<i64>().unwrap(); //字符串转i64类型
        let block_height = block_height.parse::<i64>().expect("字符串转i64类型出错");
        let block_time = block_time.parse::<i64>().expect("字符串转i64类型出错");
        let amount = BigDecimal::from_str(&amount.to_owned()).expect("msg");

        let mass_block_histories = AddMassBlockHistories {
            tx_id: tx_id,
            block_height: block_height,
            block_time: block_time,
            amount: amount,
            from_addresses: "COINBASE".to_string(),
            wallet_address: wallet.to_string(),
            pool_id: None,
            pool_name: None,
            pool_type: None,
            pool_address: None,
            status: 0,
            modify: None,
        };

        mass_block_histories.insert(&connection);

        // insert_datas.push(&mass_block_histories.clone());
        
    }

    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
    println!("退出远程SSH");
}

pub fn find_amount(outputs: &Vec<Output>, wallet: &str) -> String {
    let mut amount = String::new();
    for output in outputs {
        if output.address.eq(wallet) {
            amount = output.amount.clone();
            break;
        }
    }
    amount
}

#[derive(Debug, Deserialize)]
pub struct WalletData {
    pub chainId: String,
    pub walletId: String,
    // pub type:u8,
    pub totalBalance: String,
    pub externalKeyCount: i32,
}

#[derive(Debug, Deserialize)]
pub struct MassBlockHistories {
    pub histories: Vec<Histories>,
}

#[derive(Debug, Deserialize)]
pub struct Histories {
    pub txId: String,
    pub blockHeight: String,
    pub outputs: Vec<Output>,
    pub fromAddresses: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub address: String,
    pub amount: String,
}

/* 区块高度查到的详情结构体 */
#[derive(Debug, Deserialize)]
pub struct BlockHeight {
    pub hash: String,
    pub chainId: String,
    pub version: String,
    pub height: String,
    pub confirmations: String,
    pub time: String,
    pub previousHash: String,
    //  下面还有，只解析这么点就行
}
