use crate::common::get_ssh_session;
use crate::db::MysqlPooledConnection;
use crate::devide_coin::chia_devide::devide_chia;
use crate::dingtalk::{send, send_xch, send_xfx};
use crate::json_value::chia_block_histories_value::{ChiaData, XCHCoin};
use crate::models::chia_block_histories_model::*;
use chrono::prelude::{Local, NaiveDateTime};
use chrono::TimeZone;
use reqwest::header::HeaderMap;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::io::prelude::*;
use std::str::FromStr;

/* 定义消息发送处理结构体 */
#[derive(Debug, Clone)]
pub struct SendSms {
    pub block_height: i64,    //区块高度
    pub block_time: String,   //爆块时间
    pub pool_name: String,    //矿池名
    pub groover_name: String, //矿工名
    pub groover_count: i64,   //此矿工今日爆块
    pub amount: f64,          //获得XFX的个数
    pub calculate: String,    //计算
}

/* 通过分析js中的apiPath2=url就是接口URL */
pub fn parsing_js(js_content: String) -> Option<String> {
    let mut api_url: Option<String> = None;
    // println!("分析JS，通过分析js中的apiPath2");
    //先用;来截断
    let mut split = js_content.split(";");
    for row_code_js in split {
        /* 这一行 ：var n={apiPath:"https://api.chiaexplorer.com/chia",apiPath2:"https://will-break-dont-ever-use-this.chiaexplorer.com"}},202:function(e,a,t){e.exports=t.p+"static/media/avatar3.d5f40349.jpg"},207:function(e,a){e.exports="data:image/jpeg */
        let api_path = "n={apiPath"; //这一行里分析
        if row_code_js.find(api_path) != None {
            // println!("就是这一行：{}", row_code_js);
            let row_split = row_code_js.split(",");
            for row in row_split {
                if row.find("apiPath2:") != None {
                    // println!("处理这个:{}", row);
                    let temp = row.to_string();
                    let len = temp.len() - 3;
                    let temp = &temp[10..len];
                    // println!("这谅是找到的接口域名：{}", temp);
                    api_url = Some(temp.to_string());
                }
            }
            break;
        }
    }
    api_url
}

/* 分析取得奇亚处理块的js */
pub fn parsing_chia_main_chunk_js(html: String) -> String {
    use nipper::Document;
    /* 开始分析html */
    let document = Document::from(&html);
    let mut temp = String::new();
    document.select("script").iter().for_each(|athing| {
        let href = athing.attr("src");
        match href {
            Some(urljs) => {
                // println!("找到：{}", urljs);
                temp = urljs.to_owned().to_string();
            }
            None => println!(""),
        }
        // println!("查看一下");
    });
    // println!("完成,最后一个:{}", temp);
    temp
}

pub async fn get_url_data(url: &str) -> reqwest::Response {
    use reqwest::header::HeaderMap;
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    // headers.insert("ontent-Type", "application/json".parse().unwrap());
    headers.insert("user-agent", "luck-kd".parse().unwrap());
    let response = client.get(url).headers(headers).send().await.unwrap();
    if !response.status().as_str().eq("200") {
        println!("抓取网页({})出错!", url);
    }
    response
}

/* 取得区块浏览器上的数据 */
pub async fn get_chia_data(wallet: &String) -> ChiaData {
    // println!("开始去抓奇亚官方浏览器");
    let url = "https://www.chiaexplorer.com/blockchain/search"; //奇亚官方浏览器
    let response = get_url_data(url).await;
    let html = response.text().await.expect("抓网页出错");
    let chunk_js = parsing_chia_main_chunk_js(html);
    let url = format!("https://www.chiaexplorer.com{}", chunk_js);
    // println!("main_chunk_js:{}", url);
    let response = get_url_data(url.as_str()).await;
    let js_content = response.text().await.unwrap();
    let api_url = parsing_js(js_content);
    // println!("取到了接口域名啦:{:#?}", api_url);

    // println!("请求外网:可带GET参数page=2");
    //           https://this-api-will-break-all-the-time-do-not-use-99.chiaexplorer.com/blocks
    // curl --url https://this-api-will-break-all-the-time-do-not-use-2124.chiaexplorer.com/coinsForAddress/xch17erhru5n3j67y4y3r9maxn3a9n83w4f54hcu4rgmt36awhzpt3tsmxnzn8 -H 'user-agent: vscode-restclient'
    //curl 'https://this-api-will-break-all-the-time-do-not-use-99.chiaexplorer.com/coinsForAddress/xch17erhru5n3j67y4y3r9maxn3a9n83w4f54hcu4rgmt36awhzpt3tsmxnzn8?page=1' -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:93.0) Gecko/20100101 Firefox/93.0' -H 'Accept: application/json, text/plain, */*' -H 'Accept-Language: zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2' --compressed -H 'Origin: https://www.chiaexplorer.com' -H 'Connection: keep-alive' -H 'Referer: https://www.chiaexplorer.com/' -H 'Sec-Fetch-Dest: empty' -H 'Sec-Fetch-Mode: cors' -H 'Sec-Fetch-Site: same-site' -H 'TE: trailers'
    // let url ="https://this-api-will-break-all-the-time-do-not-use-2124.chiaexplorer.com/coinsForAddress/xch17erhru5n3j67y4y3r9maxn3a9n83w4f54hcu4rgmt36awhzpt3tsmxnzn8";
    //https://will-break-dont-ever-use-this.chiaexplorer.com/coinsForAddress
    // let url = format!("https://this-api-will-break-all-the-time-do-not-use-99.chiaexplorer.com/coinsForAddress/{}", wallet);

    let mut url = String::new();
    match api_url {
        Some(temp_url) => url = format!("{}/coinsForAddress/{}", temp_url, wallet),
        None => {
            url = format!(
                "https://will-break-dont-ever-use-this.chiaexplorer.com/coinsForAddress/{}",
                wallet
            )
        }
    }

    // let url = format!(
    //     "https://will-break-dont-ever-use-this.chiaexplorer.com/coinsForAddress/{}",
    //     wallet
    // );
    println!("取得数据的接口URL：{}", url);

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    // headers.insert("ontent-Type", "application/json".parse().unwrap());
    headers.insert("user-agent", "luck-kd".parse().unwrap());

    let response = client.get(&url).headers(headers).send().await.unwrap();
    // let kda = response.status().as_str();

    if response.status().as_str().eq("403") {
        println!("阿里云服务器中访问不了奇亚区块浏览器接口");
        let ip = "10.8.0.165:22"; //可登录的钱包节点IP
        let session = get_ssh_session(ip); //取得登录SESSION

        /* 这里执行切换钱包ID */
        let mut channel = session.channel_session().unwrap();
        let mut output = String::new();
        let command = format!("curl --url {} -H 'user-agent: mykdfaii'", url);
        println!("钱包机执行：{}", command);
        channel.exec(&command.to_owned()).unwrap();
        channel.read_to_string(&mut output).unwrap();
        let temp_chia: ChiaData =
            serde_json::from_str(&output.to_owned()).expect("出错了，解析区块高度json文件 ");
        return temp_chia;
    } else {
        println!("成功请求到了官方浏览暒");
        let temp_chia = response
            .json::<ChiaData>()
            .await
            .expect("处理区块浏览器接口数据转换成json出错");
        return temp_chia;
    }
}

pub async fn get_chiaexplorer(
    pool_id: i64,
    pool_name: &String,
    pool_order: i32,
    groover_id: i64,
    groover_name: &String,
    wallet: &String,
    is_allot: i8,
    connection: &MysqlPooledConnection,
) {
    // println!("请求外网:可带GET参数page=2");

    // curl --url https://this-api-will-break-all-the-time-do-not-use-2124.chiaexplorer.com/coinsForAddress/xch17erhru5n3j67y4y3r9maxn3a9n83w4f54hcu4rgmt36awhzpt3tsmxnzn8 -H 'user-agent: vscode-restclient'
    // let url ="https://this-api-will-break-all-the-time-do-not-use-2124.chiaexplorer.com/coinsForAddress/xch17erhru5n3j67y4y3r9maxn3a9n83w4f54hcu4rgmt36awhzpt3tsmxnzn8";
    // let url = format!("https://this-api-will-break-all-the-time-do-not-use-2124.chiaexplorer.com/coinsForAddress/{}", wallet);
    // println!("取得数据的接口URL：{}", url);

    // let client = reqwest::Client::new();
    // let mut headers = HeaderMap::new();
    // // headers.insert("ontent-Type", "application/json".parse().unwrap());
    // headers.insert("user-agent", "luck-kd".parse().unwrap());

    // let response = client.get(url).headers(headers).send().await.unwrap();
    // if response.status().as_str().eq("403") {
    //     println!("阿里云服务器中访问不了奇亚区块浏览器接口");
    // }
    // let temp_chia = response
    //     .json::<ChiaData>()
    //     .await
    //     .expect("处理区块浏览器接口数据转换成json出错");

    let temp_chia = get_chia_data(wallet).await;

    let mut send_array: HashMap<i64, SendSms> = HashMap::new();

    for xch_coin in temp_chia.coins {
        /* 先用coin_id去查找爆块表是否有记录了，没有再写入表，并准备发送消息 */
        let coin_id = &xch_coin.coin_id;
        if !xch_coin.coinbase {
            println!("非爆块，跳过处理");
            continue;
        }

        let temp_id = get_coin_id(coin_id, connection);
        if temp_id != 0 {
            println!("跳过处理");
            continue;
        }

        dispose_xch(
            &xch_coin,
            pool_id,
            pool_name,
            pool_order,
            groover_id,
            groover_name,
            wallet,
            is_allot,
            connection,
            &mut send_array,
        );
    }

    let mut message = String::new(); //定义要发送的消息提醒
    println!("处理完成,发送消息");
    if !send_array.is_empty() {
        for (_, send_sms) in send_array {
            let temp = &format!(
                "\n \r\n  矿池：{}（{}）爆块了，\n区块高度{},\n时间：{}，\n获得：{} XCH",
                send_sms.pool_name,
                send_sms.groover_name,
                send_sms.block_height,
                send_sms.block_time,
                send_sms.calculate
            );
            message.push_str(temp);

            let temp = &format!("\n此矿工今日爆块：{}个", send_sms.groover_count);
            message.push_str(temp);
        }
    }

    if !message.is_empty() {
        let temp = send_xch(message).await; //测试时先取沙下发送钉钉消息
        println!("发送消息结果：{:?}", temp);
    }
}

/* XCH爆块通知:https://oapi.dingtalk.com/robot/send?access_token=8ab7bcaf160e74f02b4cf14c7506c39fef80bf37bbe7edb508e01811cce9e42d */
pub fn dispose_xch(
    xch: &XCHCoin,
    pool_id: i64,
    pool_name: &String,
    pool_order: i32,
    groover_id: i64,
    groover_name: &String,
    wallet: &String,
    is_allot: i8,
    connection: &MysqlPooledConnection,
    send_array: &mut HashMap<i64, SendSms>,
) {
    let mut coinbase = String::new();
    if xch.coinbase {
        coinbase.push_str("true");
    } else {
        coinbase.push_str("false");
    }
    let spent = true_false_to_string(xch.spent);
    let timestamp = xch.timestamp.parse::<i64>().expect("字符串转i64类型出错");

    //创建时间示例
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now();
    let dft = now.format(fmt);
    let str_date = dft.to_string();
    // println!("当前时间：{}", str_date);
    let now_date_time = NaiveDateTime::parse_from_str(str_date.as_str(), fmt).unwrap();

    println!("这里处理插入爆块信息到表");
    let new_chia_block = AddChiaBlockHistories {
        coin_id: xch.coin_id.clone(),
        puzzle_hash: Some(xch.puzzle_hash.clone()),
        parent_coin_info: Some(xch.parent_coin_info.clone()),
        amount: Some(xch.amount),
        coinbase: Some(coinbase.clone()),
        spent: Some(spent.clone()),
        spent_block_index: Some(xch.spent_block_index),
        timestamp: Some(timestamp),
        block_header_hash: Some(xch.block_header_hash.clone()),
        block_height: Some(xch.block_height),
        pool_id: pool_id,
        pool_name: Some(pool_name.clone()),
        pool_type: Some("chia".to_string()),
        groover_id: groover_id,
        pool_order: Some(pool_order),
        groover: Some(groover_name.clone()),
        wallet_id: None,
        wallet_address: Some(wallet.clone()),
        status: 0,
        modify: None,
        create_time: None,
    };

    let row = new_chia_block.insert(connection);
    if row > 0 {
        println!("插入爆块信息成功!!,准备要发送的消息");
        // let amount = xch.amount / (1000000000000);   //得到的币数0.25个,12位
        let amount = xch.amount as f64;
        let amount = amount / 1000000000000.; //得到的币数0.25个,12位
                                              // println!("数量：{}", amount);

        match send_array.get(&xch.block_height) {
            Some(old_send_sms) => {
                let groover_count = get_groover_day_count(groover_id, connection); //取得今日爆块个数
                let amount_new = amount + old_send_sms.amount;

                send_array.insert(
                    xch.block_height,
                    SendSms {
                        block_height: xch.block_height,
                        groover_count: groover_count / 2,
                        pool_name: pool_name.clone(),
                        groover_name: groover_name.clone(),
                        block_time: old_send_sms.block_time.clone(),
                        amount: amount_new,
                        calculate: format!(" {}+{} = {}", old_send_sms.amount, amount, amount_new),
                    },
                );
            }
            None => {
                let block_time = Local.timestamp(timestamp, 0);
                send_array.insert(
                    xch.block_height,
                    SendSms {
                        block_height: xch.block_height,
                        groover_count: 0,
                        pool_name: pool_name.clone(),
                        groover_name: groover_name.clone(),
                        block_time: block_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                        amount: amount,
                        // calculate: "".to_string(),
                        calculate: amount.to_string(),
                    },
                );
            }
        }

        if is_allot.eq(&1) {
            println!("先去取得矿池的分币比例（coin_pool_ratio），再按比例分配,分币");
            devide_chia(&new_chia_block, connection);
        }
    }
}

/* bool转字符 */
pub fn true_false_to_string(check: bool) -> String {
    if check {
        "true".to_string()
    } else {
        "false".to_string()
    }
}
