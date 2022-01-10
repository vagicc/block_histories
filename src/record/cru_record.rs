//  这里定时任务记录CRU算力： 0、6、12、18、24（0）
use crate::common::*;
use crate::db::MysqlPooledConnection;
use crate::json_value::cru_work_value::*;
use crate::models::coin_pool_groover_model::*;
use crate::models::cru_hashrate_record_model::*;
use crate::models::cru_server_system_model::*;
use crate::models::cru_server_system_new_model::*;
use crate::models::server_hosting_model::*;
use crate::server_hosting_monitor::cru_hosting::*;
use chrono::prelude::{Local, NaiveDateTime};
use chrono::NaiveDate;
use hyper::http::response;
use std::collections::HashMap;

pub async fn cru_record(connection: &MysqlPooledConnection) {
    println!("每隔6小时记录所有CRU矿池和矿机算力");

    let groover_data = get_pool_type_groover("cru", connection);

    for groover in groover_data {
        let wallet_address = groover.wallet_address.as_ref().unwrap();
        let (generated_at, count, members_list) = get_cru_group_members(wallet_address).await;
        if count < 1 {
            continue;
        }

        match members_list {
            Some(members) => {
                println!("处理去取得此矿池总算力");
                let (generated_at, pool_cap_data) = get_cru_pool_cap(wallet_address).await;
                let cru_hosting = get_coin_type("cru", connection);

                if cru_hosting.is_empty() {
                    println!("请增加cru矿机");
                }

                let cru_hosting = change_hosting(cru_hosting);
                insert_cru_hashrate_record(
                    groover,
                    generated_at,
                    pool_cap_data,
                    cru_hosting,
                    members,
                    connection,
                );
            }
            None => {}
        }
    }
}

pub fn insert_cru_hashrate_record(
    groover: CoinPoolGrooverData,
    generated_at: i64,
    pool_cap_data: PoolCapData,
    cru_hosting: HashMap<String, ServerHostingData>,
    members: Vec<Member>,
    connection: &MysqlPooledConnection,
) {
    println!("开始组装数据");
    //创建时间示例
    let now = Local::now();
    let dft = now.format("%Y-%m-%d %H:%M:%S");
    let hour = now.format("%H").to_string();
    let hour = hour.parse::<i8>().expect("字符串转i8出错");
    // println!("当前的小时数：{}", hour);
    let str_date_time = dft.to_string();
    // println!("当前时间日期：{}", str_date_time);
    let now_date_time =
        NaiveDateTime::parse_from_str(str_date_time.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
    let str_date = now.format("%Y-%m-%d").to_string();
    // println!("当前日期：{}", str_date);
    let now_data = NaiveDate::parse_from_str(str_date.as_str(), "%Y-%m-%d").unwrap();

    // let pool_id = groover.pool_id;
    // let pool_name = groover.pool_name.clone();
    // let pool_type = Some("cru".to_string());
    // let groover_id = groover.id;
    // let pool_order = groover.pool_order.clone();
    // let groover_name = groover.groover.clone();
    // let wallet_id = groover.wallet_id.clone();
    // let wallet_address = groover.wallet_address.clone();

    // let pool_power = Some(pool_cap_data.power.clone());
    // let pool_limit_stake = Some(pool_cap_data.limit_stake.clone());
    // let pool_total_stake = Some(pool_cap_data.total_stake.clone());
    // let pool_members = Some(pool_cap_data.members);
    // let pool_cap = Some(pool_cap_data.cap.clone());

    for member in members {
        let account_id = member.account_id.clone();
        // let cap = Some(member.cap);
        // let used = Some(member.used);
        // let spare = Some(member.spare);
        // let report_slot = member.report_slot;
        // let eported_files_size=Some(member.reported_files_size);

        match cru_hosting.get(&account_id) {
            Some(hosting) => {
                println!("找到对应的矿机了");
                let server_id = hosting.id as u32;
                let customer_id = hosting.customer_id as i64;

                let new_record = AddcruHashrateRecord {
                    daily: now_data,
                    hours: hour,
                    pool_id: groover.pool_id,
                    pool_name: groover.pool_name.clone(),
                    pool_type: Some("cru".to_string()),
                    groover_id: groover.id,
                    pool_order: groover.pool_order.clone(),
                    groover: groover.groover.clone(),
                    wallet_id: groover.wallet_id.clone(),
                    wallet_address: groover.wallet_address.clone(),
                    status: 0,
                    pool_power: Some(pool_cap_data.power.clone()),
                    pool_limit_stake: Some(pool_cap_data.limit_stake.clone()),
                    pool_total_stake: Some(pool_cap_data.total_stake.clone()),
                    pool_members: Some(pool_cap_data.members),
                    pool_cap: Some(pool_cap_data.cap.clone()),
                    server_id: server_id,
                    customer_id: customer_id,
                    account_id: Some(account_id.clone()),
                    cap: Some(member.cap),
                    used: Some(member.used),
                    spare: Some(member.spare),
                    report_slot: member.report_slot,
                    reported_files_size: Some(member.reported_files_size),
                    generated_at: Some(generated_at),
                    update_time: now_date_time,
                };
                let _insert_id = new_record.insert(connection);
            }
            None => {
                println!("矿机({})还没有增加到表", account_id);
            }
        }
    }
}

pub async fn get_cru_pool_cap(group_owner: &String) -> (i64, PoolCapData) {
    use reqwest::header::HeaderMap;
    use serde_derive::{Deserialize, Serialize};

    let url = "https://crust.api.subscan.io/api/scan/swork/group";
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("user-agent", "luck".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    #[derive(Hash, Eq, PartialEq, Debug, Deserialize, Serialize)]
    struct ContentData {
        group_owner: String,
    }
    let content = ContentData {
        group_owner: group_owner.clone(),
    };

    println!("请求的URL:{}", url);
    println!("请求的group_owner:{}", group_owner.clone());

    let response = client
        .post(url)
        .headers(headers)
        .json(&content)
        .send()
        .await
        .unwrap();

    if !response.status().as_str().eq("200") {
        // println!("请求接口出错", url);
    }

    let data = response.json::<GroupOwnerData>().await.unwrap();

    let generated_at = data.generated_at;
    let pool_data = data.data;
    (generated_at, pool_data)
}
