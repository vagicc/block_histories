use crate::common::*;
use crate::db::MysqlPooledConnection;
use crate::dingtalk;
use crate::json_value::cru_work_value::*;
use crate::models::coin_pool_groover_model::get_pool_type_groover;
use crate::models::coin_pool_model::get_pool_wallet;
use crate::models::cru_server_system_model::*;
use crate::models::cru_server_system_new_model::*;
use crate::models::server_hosting_model::*;
use chrono::prelude::{Local, NaiveDateTime};
use std::collections::HashMap;
use std::io::{Read, Write};

pub async fn cru_server_new(connection: &MysqlPooledConnection) {
    let cru_hosting = get_coin_type("cru", connection);

    if cru_hosting.is_empty() {
        println!("请增加cru矿机");
    }
    let cru_hosting = change_hosting(cru_hosting);

    let groover_data = get_pool_type_groover("cru", connection);

    for groover in groover_data {
        let wallet_address = groover.wallet_address.as_ref().unwrap();
        let (generated_at, count, members_list) = get_cru_group_members(wallet_address).await;
        if count < 1 {
            continue;
        }

        println!("开始处理插入或更新数据");
        match members_list {
            Some(members) => {
                insert_or_update(
                    cru_hosting.clone(),
                    wallet_address,
                    generated_at,
                    members,
                    connection,
                )
                .await;
            }
            None => {}
        }
        // let members_list = members_list.unwrap();
        // insert_or_update(
        //     cru_hosting.clone(),
        //     wallet_address,
        //     generated_at,
        //     members_list,
        //     connection,
        // )
        // .await;
    }
}

pub async fn insert_or_update(
    hostings: HashMap<String, ServerHostingData>,
    group_owner: &String,
    generated_at: i64,
    members_list: Vec<Member>,
    connection: &MysqlPooledConnection,
) {
    //创建时间示例
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now();
    let dft = now.format(fmt);
    let str_date = dft.to_string();
    // println!("当前时间：{}", str_date);
    let now_date_time = NaiveDateTime::parse_from_str(str_date.as_str(), fmt).unwrap();

    for member in members_list {
        let account_id = member.account_id;
        let mut hint = format!("CRU矿机预警:\n account_id:\n{}", account_id);

        let mut server_id = 0;
        let mut customer_id = 0;
        let mut ip1: Option<String> = None;
        let mut ip2: Option<String> = None;

        match hostings.get(&account_id) {
            Some(hosting) => {
                server_id = hosting.id as u32;
                customer_id = hosting.customer_id as i64;
                ip1 = hosting.ip1.clone();
                ip2 = hosting.ip2.clone();
                hint = format!("\n {}IP({:?})", hint, hosting.ip1.clone());
            }
            None => {}
        }

        let cru_server_system = AddCruServerSystem {
            server_id: server_id,
            customer_id: customer_id,
            ip1: ip1.clone(),
            ip2: ip2.clone(),
            group_owner: Some(group_owner.clone()),
            account_id: Some(account_id.clone()),
            cap: Some(member.cap.clone()),
            used: Some(member.used.clone()),
            spare: Some(member.spare.clone()),
            report_slot: member.report_slot,
            reported_files_size: Some(member.reported_files_size.clone()),
            generated_at: Some(generated_at),
            chain_status: None,
            api_status: None,
            sworker_status: None,
            smanager_status: None,
            ipfs_status: None,
            create_time: Some(now_date_time),
        };
        let insert_id = cru_server_system.insert(connection);

        let cru_new = AddCruServerSystemNew {
            server_id: server_id,
            customer_id: customer_id,
            ip1: ip1,
            ip2: ip2,
            group_owner: Some(group_owner.clone()),
            account_id: Some(account_id.clone()),
            cap: Some(member.cap.clone()),
            used: Some(member.used.clone()),
            spare: Some(member.spare.clone()),
            report_slot: member.report_slot,
            reported_files_size: Some(member.reported_files_size.clone()),
            generated_at: Some(generated_at),
            // srd_complete: srd.srd_complete,
            // srd_remaining_task: srd.srd_remaining_task,
            // disk_available_for_srd: srd.disk_available_for_srd,
            // disk_available: srd.disk_available,
            // disk_volume: srd.disk_volume,
            // sys_disk_available: srd.sys_disk_available,
            // disk_count: disk_count,
            chain_status: None,
            api_status: None,
            sworker_status: None,
            smanager_status: None,
            ipfs_status: None,
            update_time: Some(now_date_time),
        };

        let cru_server_system_new = get_account_id(&account_id, connection);
        match cru_server_system_new {
            Some(system_new) => {
                // println!("已有数据,进行数据比对，看是否要发警告到钉钉消息,然后更新数据到表");
                // println!("Old system:{:#?}", system_new);
                let _updated_row = cru_new.update(system_new.id, connection);
                let old_cap = system_new.cap;
                let cap = member.cap.clone();
                let cap = cap.parse::<u64>().expect("字符串转U64位出错");
                match old_cap {
                    Some(oldcap) => {
                        let oldcap = oldcap.parse::<u64>().expect("字符串转U64位出错");
                        match oldcap.checked_sub(cap) {
                            Some(sub) => {
                                let temp: u64 = 1024 * 1024 * 1024 * 1024 * 10;
                                if sub > temp {
                                    println!(
                                        "算力比以前小了，得警告到钉钉,上次算力{}，此次算力{}",
                                        oldcap, cap
                                    );
                                    hint = format!(
                                        "{}\n算力下跌 \n上次算力{} \n此次算力{}\n",
                                        hint,
                                        show_cru_spaces(oldcap),
                                        show_cru_spaces(cap)
                                    );
                                    dingtalk::send_hosting(hint).await;
                                }
                            }
                            None => {}
                        }
                        
                    }
                    None => {}
                }
            }
            None => {
                println!("暂无数据，新增数据");
                let _row = cru_new.insert(connection);
            }
        }
    }
}

pub fn show_cru_spaces(spaces: u64) -> String {
    let mut show_spqces = String::new();
    if spaces >= 1024 * 1024 * 1024 * 1024 * 1024 * 1024 {
        let spaces = spaces as f64;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        show_spqces = format!("{}EB", spaces);
        // show_spqces = format!("{}EB", spaces / (1024 * 1024 * 1024 * 1024 * 1024 * 1024));
    } else if spaces >= 1024 * 1024 * 1024 * 1024 * 1024
        && spaces < 1024 * 1024 * 1024 * 1024 * 1024 * 1024
    {
        let spaces = spaces as f64;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        show_spqces = format!("{}PB", spaces);
        // show_spqces = format!("{}PB", spaces / (1024 * 1024 * 1024 * 1024 * 1024));
    } else if spaces >= 1024 * 1024 * 1024 * 1024 && spaces < 1024 * 1024 * 1024 * 1024 * 1024 {
        let spaces = spaces as f64;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        let spaces = spaces / 1024.;
        show_spqces = format!("{}TB", spaces);
        // show_spqces = format!("{}TB", spaces / (1024 * 1024 * 1024 * 1024));
    } else if spaces >= 1024 * 1024 * 1024 && spaces < 1024 * 1024 * 1024 * 1024 {
        let spaces = spaces as f64;
        let t = (1024 * 1024 * 1024) as f64;
        show_spqces = format!("{}GB", spaces / t);
        // show_spqces = format!("{}GB", spaces / (1024 * 1024 * 1024));
    } else {
        show_spqces = format!("{}MB", spaces / (1024 * 1024));
    }
    show_spqces
}

pub fn change_hosting(cru_hosting: Vec<ServerHostingData>) -> HashMap<String, ServerHostingData> {
    let mut change_data: HashMap<String, ServerHostingData> = HashMap::new();

    for hosting in cru_hosting {
        // let key = hosting.account_id.as_ref();
        match hosting.account_id.as_ref() {
            Some(key) => {
                change_data.insert(key.clone(), hosting);
            }
            None => {}
        }
    }

    change_data
}

/* 查询每台矿机的算力 */
pub async fn get_cru_group_members(group_owner: &String) -> (i64, i32, Option<Vec<Member>>) {
    use reqwest::header::HeaderMap;
    use serde_derive::{Deserialize, Serialize};

    let url = "https://crust.api.subscan.io/api/scan/swork/group/members";
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("user-agent", "luck".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    #[derive(Hash, Eq, PartialEq, Debug, Deserialize, Serialize)]
    struct ContentData {
        row: i32,
        page: i32,
        group_owner: String,
    }
    let content = ContentData {
        row: 1000,
        page: 0,
        group_owner: group_owner.clone(),
    };

    let response = client
        .post(url)
        .headers(headers)
        .json(&content)
        .send()
        .await
        .unwrap();

    if !response.status().as_str().eq("200") {
        println!("请求接口({})出错!", url);
    }

    let data = response
        .json::<GroupMembersData>()
        .await
        .unwrap_or_else(|op| {
            println!("这个地址出错了：{}", group_owner);
            println!("{:#?}", op);
            panic!("=============================================");
        });

    // let data = response
    //     .json::<GroupMembersData>()
    //     .await
    //     .expect("接口返回来数据转换成JSON出错");
    // println!("取得数据成功：{:#?}", data);

    let generated_at = data.generated_at;
    let members_data = data.data;
    let count = members_data.count;
    let members_list = members_data.list;
    (generated_at, count, members_list)
}

pub fn _cru_server(connection: &MysqlPooledConnection) {
    println!("先去取所有的CRU类矿机，再按IP去取得工作量，更新到表");
    let cru_hosting = get_coin_type("cru", connection);

    if cru_hosting.is_empty() {
        println!("请增加cru矿机");
    }

    for server_hosting in cru_hosting {
        if server_hosting.ip1 == None {
            println!("矿机IP1不能为空");
            continue;
        }
        cru_main_logic(&server_hosting, connection); //旧的流程
    }
}

pub fn cru_main_logic(server_hosting: &ServerHostingData, connection: &MysqlPooledConnection) {
    let ip = server_hosting.ip1.as_ref().unwrap().to_owned();
    // println!("ip1:{}",&server_hosting.ip1.unwrap());
    let cru_work_data = get_cru_work_data(&ip); //取得工作量上报
                                                // println!("work data:{:#?}", cru_work_data);
    let mut service_status: HashMap<String, String> = HashMap::new();
    get_service_status(&ip, &mut service_status); //取得矿机状态
                                                  // println!("服务状态:{:#?}", service_status);
    println!("插数据到表就行");

    let srd = cru_work_data.srd;

    let server_id = server_hosting.id as u32;
    let customer_id = server_hosting.customer_id as i64;
    // let ip1 = server_hosting.ip1.clone();
    // let ip2 = server_hosting.ip2.clone();

    // let srd_complete = srd.srd_complete;
    // let srd_remaining_task = srd.srd_remaining_task;
    // let disk_available_for_srd = srd.disk_available_for_srd;
    // let disk_available = srd.disk_available;
    // let disk_volume = srd.disk_volume;
    // let sys_disk_available = srd.sys_disk_available;
    let disk_count = srd.srd_detail.len() as i32;

    let chain_status = service_status.get("chain").unwrap().clone();
    let api_status = service_status.get("api").unwrap().clone();
    let sworker_status = service_status.get("sworker").unwrap().clone();
    let smanager_status = service_status.get("smanager").unwrap().clone();
    let ipfs_status = service_status.get("ipfs").unwrap().clone();

    //创建时间示例
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now();
    let dft = now.format(fmt);
    let str_date = dft.to_string();
    // println!("当前时间：{}", str_date);
    let now_date_time = NaiveDateTime::parse_from_str(str_date.as_str(), fmt).unwrap();

    let cru_server_system = AddCruServerSystem {
        server_id: server_id,
        customer_id: customer_id,
        ip1: server_hosting.ip1.clone(),
        ip2: server_hosting.ip2.clone(),

        group_owner: None,
        account_id: None,
        cap: None,
        used: None,
        spare: None,
        report_slot: 0,
        reported_files_size: None,
        generated_at: None,

        // srd_complete: srd.srd_complete,
        // srd_remaining_task: srd.srd_remaining_task,
        // disk_available_for_srd: srd.disk_available_for_srd,
        // disk_available: srd.disk_available,
        // disk_volume: srd.disk_volume,
        // sys_disk_available: srd.sys_disk_available,
        // disk_count: disk_count,
        chain_status: Some(chain_status.clone()),
        api_status: Some(api_status.clone()),
        sworker_status: Some(sworker_status.clone()),
        smanager_status: Some(smanager_status.clone()),
        ipfs_status: Some(ipfs_status.clone()),
        create_time: Some(now_date_time),
    };

    let _row = cru_server_system.insert(connection);

    let cru_new = AddCruServerSystemNew {
        server_id: server_id,
        customer_id: customer_id,
        ip1: server_hosting.ip1.clone(),
        ip2: server_hosting.ip2.clone(),

        group_owner: None,
        account_id: None,
        cap: None,
        used: None,
        spare: None,
        report_slot: 0,
        reported_files_size: None,
        generated_at: None,

        // srd_complete: srd.srd_complete,
        // srd_remaining_task: srd.srd_remaining_task,
        // disk_available_for_srd: srd.disk_available_for_srd,
        // disk_available: srd.disk_available,
        // disk_volume: srd.disk_volume,
        // sys_disk_available: srd.sys_disk_available,
        // disk_count: disk_count,
        chain_status: Some(chain_status),
        api_status: Some(api_status),
        sworker_status: Some(sworker_status),
        smanager_status: Some(smanager_status),
        ipfs_status: Some(ipfs_status),
        update_time: Some(now_date_time),
    };

    let cru_server_system_new = get_ip(&ip, connection);
    match cru_server_system_new {
        Some(system_new) => {
            println!("已有数据,进行数据比对，看是否要发警告到钉钉消息,然后更新数据到表");
            // println!("Old system:{:#?}", system_new);
            let _updated_row = cru_new.update(system_new.id, connection);
        }
        None => {
            println!("暂无数据，新增数据");
            let _row = cru_new.insert(connection);
        }
    }
}

/* 取得服务状态 */
pub fn get_service_status(ip: &str, service_status: &mut HashMap<String, String>) {
    let manage_ip = "14.29.126.40:22"; //可登录的钱包节点IP
    let mut session = get_ssh_session_passwd(manage_ip, "root", "QWer@1234.."); //取得登录SESSION
    let mut channel = session.channel_session().unwrap();

    // let mut service_status: HashMap<&str, &str> = HashMap::new();

    let mut output = String::new();

    let command = format!("sshpass -p 'QWer@1234..' ssh root@{} crust status", ip);
    println!("执行的命令：{}", command);
    let mut channel = session.channel_session().unwrap();
    channel.exec(&command).unwrap();
    channel.read_to_string(&mut output).unwrap();
    // println!("矿机服务状态：{:#?}", output);
    let status_vec = output.split("\n");
    for status_row in status_vec {
        if status_row.find("-") != None || status_row.find("Service") != None {
            continue;
        }
        // println!("{}", status_row);
        let service: Vec<&str> = status_row.split(' ').filter(|x| !x.is_empty()).collect();
        // println!("{:#?}", service);
        if service.len() == 2 {
            // println!("服务{}的状态是{}", service[0], service[1]);
            // service_status.insert(service[0], service[1]);
            service_status.insert(service[0].to_owned(), service[1].to_owned());
        }
    }
    // println!("{:#?}", service_status);
}

/* 取得工作量上报（这里是登录到矿机的管理机上执行命令得到） */
pub fn get_cru_work_data(ip: &str) -> CRUWork {
    let manage_ip = "14.29.126.40:22"; //可登录的钱包节点IP
    let mut session = get_ssh_session_passwd(manage_ip, "root", "QWer@1234.."); //取得登录SESSION
    let mut channel = session.channel_session().unwrap();

    // println!("three");
    let mut output = String::new();

    let command = format!(
        "sshpass -p 'QWer@1234..' ssh root@{} crust tools workload",
        ip
    );
    println!("执行的命令：{}", command);
    // let command = "sshpass -p 'QWer@1234..' ssh root@10.10.2.28 crust tools workload";
    let mut channel = session.channel_session().unwrap();
    channel.exec(&command).unwrap();
    channel.read_to_string(&mut output).unwrap();
    // println!("里而矿机的执行命令的结果：{:#?}", output);

    let work_data: CRUWork = serde_json::from_str(&output.to_owned()).expect("解析工作量Json出错");
    return work_data;
    // println!("Work:{:#?}", work_data);
    // let srd_count = work_data.srd.srd_detail.len();
    // println!("硬盘数量：{}", srd_count);
}
