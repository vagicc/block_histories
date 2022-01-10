use crate::db::MysqlPooledConnection;
use crate::dingtalk::send_cru;
use crate::json_value::cru_block_hitories_value::*;
use crate::models::coin_pool_groover_model::{get_pool_type_groover, CoinPoolGrooverData};
use crate::models::cru_block_histories_model::*;
use crate::models::find_block_histories_model::*;
use async_recursion::async_recursion;
use chrono::prelude::{Local, NaiveDateTime};
use chrono::TimeZone;
use hyper::http::response;
use rustc_serialize::json::ToJson;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/* cru币主逻辑 */
pub async fn logic(connection: &MysqlPooledConnection) -> bool {
    println!("处理cru币收益主逻辑");

    /* 先从任务调度里取得开始区块，并且插入可并行的任务,再去查找区块:从区块：955500 开始扫*/
    // let find_block = get_start_block("cru", connection);
    let find_block = get_start_block("cru", connection).expect("注意无任务调度，看下表^^^^");

    let mut block_num = find_block.block_num.unwrap();
    let cycles_num = 1900; //此次查找区块量
    let new_block_num = block_num + cycles_num; //此次扫块的终点
    let cru_block_num = get_cru_block_num().await;

    /* 确保区块扫描不大于最大的区块高度 */
    let mut next_block_num = new_block_num;
    if new_block_num >= cru_block_num {
        next_block_num = cru_block_num - 500;
    }

    let new_find_block = NewFindBlockHistories {
        coin_type: find_block.coin_type.clone(),
        block_num: next_block_num,
        cycles_num: cycles_num,
        status: 0,
        optimism: None,
        modify: None,
        create_time: None,
    };

    if new_find_block.insert(connection) < 1 {
        println!("插入新的任务调度出错，新数据：{:#?}", new_find_block);
    }

    let groover_data = get_pool_type_groover("cru", connection);
    if groover_data.is_empty() {
        println!("你还没有设置CRU矿池的矿工哦");
        return false;
    }

    let groover_data = with_groover_data(groover_data);

    while block_num <= new_block_num {
        let extrinsics = get_cru_extrinsics(block_num, 0).await; //取得交易号
        block_num += 1; //区块自增

        if extrinsics.is_empty() {
            break;
        }

        let extrinsic_index_data = find_account_id(extrinsics, &groover_data);
        if extrinsic_index_data.is_empty() {
            println!("元对应的钱包交易号");
            continue;
        }

        /* 用交易号去取得交易详情，然后写到收益表，并提示钉钉消息有收益了 */
        for (extrinsic_index, groover) in extrinsic_index_data {
            let cru_data = get_cru_extrinsic_index(&extrinsic_index, &groover).await;
            /* 取到了数据，可以去表查找是否有记录过，如没记录过再记录并且发钉钉消息提示 */
            cru_block_histories(&cru_data, &groover, connection).await;
        }
    }
    //更新此次任务为已完成
    let updated_row = set_status(find_block.id, 2, find_block.optimism + 1, connection);

    if updated_row <= 0 {
        println!("更新任务状态为已执行完失败:{:#?}", find_block);
        return false;
    }
    return true;
}

pub async fn _logic_old(connection: &MysqlPooledConnection) -> bool {
    println!("处理cru币收益主逻辑");
    let groover_data = get_pool_type_groover("cru", connection);
    if groover_data.is_empty() {
        println!("你还没有设置CRU矿池的矿工哦");
        return false;
    }

    let groover_data = with_groover_data(groover_data);

    let mut block_num = 895211;
    loop {
        let extrinsics = get_cru_extrinsics(block_num, 0).await; //取得交易号
        block_num += 1; //区块自增

        if extrinsics.is_empty() {
            break;
        }

        let extrinsic_index_data = find_account_id(extrinsics, &groover_data);
        if extrinsic_index_data.is_empty() {
            println!("元对应的钱包交易号");
            continue;
        }

        /* 用交易号去取得交易详情，然后写到收益表，并提示钉钉消息有收益了 */
        for (extrinsic_index, groover) in extrinsic_index_data {
            let cru_data = get_cru_extrinsic_index(&extrinsic_index, &groover).await;
            /* 取到了数据，可以去表查找是否有记录过，如没记录过再记录并且发钉钉消息提示 */
            cru_block_histories(&cru_data, &groover, connection).await;
        }
    }

    return true;
}

#[derive(Debug, Clone)]
pub struct SendSms {
    pub pool_name: String,       //矿池名
    pub groover_name: String,    //矿工名
    pub groover_count: i64,      //此矿工今日爆块
    pub amount: f64,             //获得XFX的个数
    pub calculate: String,       //计算:  kk+aa=ee
    pub extrinsic_index: String, //区块数字，交易号
    pub block_time: String,      //爆块时间
}

pub async fn cru_block_histories(
    cru_index_data: &Vec<CRUIndexData>,
    groover: &CoinPoolGrooverData,
    connection: &MysqlPooledConnection,
) {
    println!("查找是否有记录过，如没记录过再记录并且发钉钉消息提示");
    let mut send_array: HashMap<String, SendSms> = HashMap::new();
    let mut pool_data: HashMap<i64, i8> = HashMap::new();

    for index_data in cru_index_data {
        // `event_idx` INT DEFAULT NULL COMMENT '相同的extrinsic_index(block_num连extrinsic_idx)，不同的这个区分不同获利',
        let id = check_exist(
            &index_data.extrinsic_index,
            index_data.event_idx,
            connection,
        );

        if id != 0 {
            println!("此收益之前已记录过");
            continue;
        }

        println!("还没记录，记录到表，并提示钉钉消息");
        let pool_name = groover.pool_name.as_ref().unwrap();
        let wallet_id = groover.wallet_id.clone();

        let new_data = AddCruBlock {
            extrinsic_index: index_data.extrinsic_index.clone(),
            block_timestamp: Some(index_data.block_timestamp),
            generated_at: Some(index_data.generated_at),
            signature: Some(index_data.signature.clone()),
            extrinsic_hash: index_data.extrinsic_hash.clone(),
            block_num: Some(index_data.block_num),
            extrinsic_idx: Some(index_data.extrinsic_idx),
            module_id: Some(index_data.module_id.clone()),
            event_id: Some(index_data.event_id.clone()),
            event_idx: Some(index_data.event_idx),
            account_id: Some(index_data.account_id.clone()),
            amount: Some(index_data.amount),
            finalized: Some(index_data.finalized.clone()),

            pool_id: groover.pool_id,
            pool_name: Some(pool_name.to_string()),
            pool_type: Some(groover.pool_type.as_ref().unwrap().to_string()),
            groover_id: groover.id,
            pool_order: Some(groover.pool_order.unwrap()),
            groover: Some(groover.groover.as_ref().unwrap().to_string()),
            wallet_id: wallet_id,
            wallet_address: Some(groover.wallet_address.as_ref().unwrap().to_string()),
            status: 0,
            modify: None,
            create_time: None,
        };
        let row = new_data.insert(connection);
        if row > 0 {
            println!("插入新的收益成功，准备要发送的消息");
            let amount = index_data.amount as f64;
            let amount = amount / 1000000000000.; //得到的币数0.257557844252 CRU个,12位
            let key = &index_data.extrinsic_index.clone();
            let block_time = Local.timestamp(index_data.block_timestamp, 0);

            let temp = &format!(
                "\r\n {}（{}）获得了收益，\n交易号：{}，event_idx:{} \n收益：{}CRU，\n时间：{} ",
                pool_name,
                groover.groover.as_ref().unwrap().to_string(),
                index_data.extrinsic_index.clone(),
                index_data.event_idx,
                amount,
                block_time.format("%Y-%m-%d %H:%M:%S").to_string()
            );
            send_cru(temp.to_string()).await;

            // 判断是否要执行自动分币
            if pool_data.is_empty() {
                pool_data = get_cru_pool_data(connection);
            }

            match pool_data.get(&groover.pool_id) {
                Some(pool_allot) => {
                    if pool_allot.eq(&1) {
                        println!("执行自动分币");
                        use crate::devide_coin::cru_devide::php_devide_cru;
                        php_devide_cru(&new_data, connection);
                    }
                }
                None => {}
            }
        }
    }
}

pub fn get_cru_pool_data(connection: &MysqlPooledConnection) -> HashMap<i64, i8> {
    use crate::models::coin_pool_model::get_type_pool;
    let pool_data = get_type_pool("cru", connection);

    let mut pool_temp: HashMap<i64, i8> = HashMap::new();
    for (id, _, _, allot) in pool_data {
        pool_temp.insert(id, allot);
    }
    pool_temp
}

pub async fn get_cru_block_num() -> i64 {
    let url = "https://crust.webapi.subscan.io/api/scan/metadata";
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("user-agent", "luck".parse().unwrap());

    let response = client
        .post(url)
        .headers(headers)
        .send()
        .await
        .expect("请求区块浏览器接口取得CRU交易详情出错"); //这里如果超时，应该再做重新再请求一次
                                                          //thread 'main' panicked at '请求区块浏览器接口取得CRU交易详情出错: reqwest::Error { kind: Request, url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("crust.webapi.subscan.io")), port: None, path: "/api/scan/extrinsic", query: None, fragment: None }, source: hyper::Error(Connect, ConnectError("tcp connect error", Os { code: 110, kind: TimedOut, message: "Connection timed out" })) }', src/block_historries/cru_block_histories.rs:74:10

    if !response.status().as_str().eq("200") {
        println!(
            "请求接口（{}）出错，状态码为：{}",
            url,
            response.status().as_str()
        );
    }

    let data = response
        .json::<CRUMetadata>()
        .await
        .expect("交易详情转换成json数据出错");

    let new_block_num = data.data.blockNum.parse::<i64>().unwrap();
    new_block_num
}

pub async fn get_cru_extrinsic_index(
    extrinsic_index: &String,
    groover: &CoinPoolGrooverData,
) -> Vec<CRUIndexData> {
    let url = "https://crust.webapi.subscan.io/api/scan/extrinsic";
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("user-agent", "luck".parse().unwrap());

    /* 设置post的参数 */
    #[derive(Debug, Deserialize, Serialize)]
    struct RequeryData {
        extrinsic_index: String,
    }
    let request_data = RequeryData {
        extrinsic_index: extrinsic_index.clone(),
    };

    let mut cru_data: Vec<CRUIndexData> = Vec::new();

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_data)
        .send()
        .await
        .expect("请求区块浏览器接口取得CRU交易详情出错"); //这里如果超时，应该再做重新再请求一次
                                                          //thread 'main' panicked at '请求区块浏览器接口取得CRU交易详情出错: reqwest::Error { kind: Request, url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("crust.webapi.subscan.io")), port: None, path: "/api/scan/extrinsic", query: None, fragment: None }, source: hyper::Error(Connect, ConnectError("tcp connect error", Os { code: 110, kind: TimedOut, message: "Connection timed out" })) }', src/block_historries/cru_block_histories.rs:74:10

    if !response.status().as_str().eq("200") {
        println!(
            "请求接口（{}）出错，状态码为：{}",
            url,
            response.status().as_str()
        );
    }

    let data = response
        .json::<CRUExtrinsic>()
        .await
        .expect("交易详情转换成json数据出错");

    // println!("交易详情DATA：{:#?}", data);

    let generated_at = data.generated_at;
    let extrinsic_data = data.data;
    let account_id = extrinsic_data.account_id;
    let block_timestamp = extrinsic_data.block_timestamp;
    let signature = extrinsic_data.signature;
    let event_data = extrinsic_data.event;
    for event in event_data {
        //只处理"module_id": "staking", 与"event_id": "Reward"，
        if !event.module_id.eq("staking") || !event.event_id.eq("Reward") {
            println!("只处理\"module_id\": \"staking\", 与\"event_id\": \"Reward\",其它的跳过");
            continue;
        }

        println!("都来这里了没有？");

        let params_json = event.params;
        if params_json.is_empty() {
            println!("为空的params_json,路过处理");
            continue;
        }
        let params: Vec<ParamData> = serde_json::from_str(&params_json).unwrap();
        // println!("params:{:#?}", params);

        let (check_accountid, amount) = get_accountid_and_amount(params);
        let mut wallet_id = String::new();
        let temp = groover.wallet_id.as_ref();
        match temp {
            Some(waid) => wallet_id = waid.clone(),
            None => {}
        }
        if !wallet_id.is_empty() && !check_accountid.eq(&wallet_id) {
            println!(
                "查看跳过的区块==================================================????????====="
            );
            println!("wallet_id: {}", &wallet_id);
            println!("accountid: {}", &check_accountid);
            continue; //这里改了程序流程,加了判断是为当前账号的才处理
        }

        // let amount = get_amount(params);   //这里改了程序流程

        let finalized = if event.finalized { "true" } else { "false" };

        let index_data = CRUIndexData {
            extrinsic_index: extrinsic_index.clone(),
            block_timestamp: block_timestamp,
            generated_at: generated_at,
            signature: signature.clone(),
            extrinsic_hash: event.extrinsic_hash,
            block_num: event.block_num,
            extrinsic_idx: event.extrinsic_idx,
            module_id: event.module_id,
            event_id: event.event_id,
            event_idx: event.event_idx,
            account_id: account_id.clone(),
            amount: amount,
            finalized: finalized.to_string(),
        };
        cru_data.push(index_data);
    }

    cru_data
}

#[derive(Debug, Clone)]
pub struct CRUIndexData {
    pub extrinsic_index: String,
    pub block_timestamp: i64,
    pub generated_at: i64,
    pub signature: String,
    pub extrinsic_hash: String,
    pub block_num: i64,
    pub extrinsic_idx: i32,
    pub module_id: String,
    pub event_id: String,
    pub event_idx: i32,
    pub account_id: String,
    pub amount: i64,
    pub finalized: String,
}

pub fn get_accountid_and_amount(params: Vec<ParamData>) -> (String, i64) {
    let mut amount: i64 = 0;
    let mut accountid = String::new();
    for data in params {
        if data.r#type.eq("Balance") {
            amount = data.value.parse::<i64>().expect("字符串转i64类型出错");
        }
        if data.r#type.eq("AccountId") {
            accountid = data.value;
        }
    }
    (accountid, amount)
}

pub fn get_amount(params: Vec<ParamData>) -> i64 {
    let mut amount: i64 = 0;
    for data in params {
        if data.r#type.eq("Balance") {
            amount = data.value.parse::<i64>().expect("字符串转i64类型出错");
        }
    }
    amount
}

/* 注意，这里是异步递归 */
#[async_recursion]
pub async fn get_cru_extrinsics(block_num: i64, page: i32) -> Vec<Extrinsics> {
    println!("从区块{}开始，去取得cru交易号", block_num);
    let url = "https://crust.webapi.subscan.io/api/scan/extrinsics";
    let row = 100; //接口每次最大只能取得100条数据，不然会出错
                   // let page = 0; //取得第一页（0为第一面，1为第2页）

    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("user-agent", "luck".parse().unwrap());

    /* 设置请求参数 */
    #[derive(Debug, Deserialize, Serialize)]
    struct RequeryData {
        row: i32,
        page: i32,
        block_num: i64,
    }
    let requery_data = RequeryData {
        row: row,
        page: page,
        block_num: block_num,
    };

    let response = client
        .post(url)
        .headers(headers)
        .json(&requery_data)
        .send()
        .await
        .unwrap();

    // println!("请求返回的状态码：{}", response.status().as_str());

    if !response.status().as_str().eq("200") {
        println!(
            "请求接口（{}）出错，状态码为：{}",
            url,
            response.status().as_str()
        );
        let temp: Vec<Extrinsics> = Vec::new();
        return temp;
    }

    let data = response
        .json::<CRUExtrinsics>()
        .await
        .expect("区块详情转换成json数据出错");

    let data = data.data;
    let count = data.count;
    let mut extrinsics = data.extrinsics.unwrap_or_else(|| {
        let temp: Vec<Extrinsics> = Vec::new();
        return temp;
    });

    let page_count = count as f64 / row as f64;
    let page_count = page_count.ceil() as i32;
    println!("总页数：{}，当前页数:{}", page_count, page);

    let page = page + 1;
    if row < count && page < page_count {
        println!("还有下一页");
        let mut child = get_cru_extrinsics(block_num, page).await;
        if !child.is_empty() {
            extrinsics.append(&mut child); //在后面追加，也是合并Vec
        }
    }
    return extrinsics;
}

pub fn find_account_id(
    extrinsics: Vec<Extrinsics>,
    groover_data: &HashMap<String, CoinPoolGrooverData>,
) -> Vec<(String, CoinPoolGrooverData)> {
    let mut extrinsic_index_and_groover: Vec<(String, CoinPoolGrooverData)> = Vec::new();
    for extrinsic in extrinsics {
        let account_id = &extrinsic.account_id;
        if account_id.is_empty() {
            println!("钱包地址为空，路过");
            continue;
        }

        match groover_data.get(account_id) {
            Some(groover) => {
                // println!("找到了交易号extrinsic_index:{}", &extrinsic.extrinsic_index);
                extrinsic_index_and_groover.push((extrinsic.extrinsic_index, groover.clone()));
            }
            None => {}
        }
    }
    extrinsic_index_and_groover
}

pub fn with_groover_data(
    groover_data: Vec<CoinPoolGrooverData>,
) -> HashMap<String, CoinPoolGrooverData> {
    let mut new_data: HashMap<String, CoinPoolGrooverData> = HashMap::new();
    for groover in groover_data.clone() {
        // let tempkey = &groover.wallet_address.unwrap().to_owned();
        // new_data.insert(tempkey, groover.clone());
        let tempkey = &groover.wallet_address;
        let tempkey = tempkey.as_ref().unwrap().to_owned();
        new_data.insert(tempkey, groover);
    }
    new_data
}
