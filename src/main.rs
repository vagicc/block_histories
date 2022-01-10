use crate::block_historries::chia_block_histories::*;
use crate::block_historries::cru_block_histories::logic;
use crate::block_historries::mass_block_histories::command_find_block_histories;
use crate::block_historries::test_block_histories::*;
use crate::block_historries::xfx_block_histories::xfx_block_to_mysql;
use crate::db::establish_connection;
use crate::db::MysqlPooledConnection;
use crate::models::coin_pool_groover_model::get_pool_groover;
use crate::models::coin_pool_model::{get_pool, get_type_pool};
// use crate::models::coin_pool_model::get_pool_wallet;
use crate::dingtalk::{send, send_xch, send_xfx};
use crate::record::cru_record;
use crate::server_hosting_monitor::cru_hosting::*;
use chrono::prelude::Local;
use chrono::TimeZone;
use glob::glob;
use std::fs;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::DateTime;
use chrono::Datelike;

mod block_historries;
mod common;
mod db;
mod devide_coin;
mod dingtalk;
mod json_value;
mod models;
mod record;
mod schema;
mod server_hosting_monitor;

#[macro_use]
extern crate diesel;

#[tokio::main]
async fn main() {
    let coin_type = std::env::args().nth(1).unwrap_or_else(|| {
        let temp = "mass".to_string();
        return temp;
    });

    // println!("取得爆块");

    let connection = establish_connection();

    if coin_type.eq("mass") {
        devide_type(&connection).await;
    } else if coin_type.eq("xfx") {
        println!("这里是处理XFX的区块高度(通过读取json文件来处理)");
        let json_path = std::env::args()
            .nth(2)
            .expect("命令第二个参数应该带XFX的json文件目录过来");

        let mut message = String::new(); //定义要发送的消息提醒

        let path = json_path.clone() + "/*.json";

        /* 创建目录，并移动文件到新的目录下 */
        let local: DateTime<Local> = Local::now();
        let new_path = format!(
            "{}/{}-{}-{}",
            &json_path,
            local.year(),
            local.month(),
            local.day()
        );
        fs::create_dir_all(&new_path).expect("创建新目录失败"); //如果不存在则创建

        for entry in glob(&path).unwrap() {
            println!("luck--");
            let path = entry.unwrap();
            let filename = path.file_name().unwrap(); //取得文件名
            let filename = filename.to_str().unwrap(); //字符文件名

            xfx_block_to_mysql(path.clone(), &connection, &mut message).await;

            let newname = new_path.clone() + "/" + filename;
            let oldname = format!("{}/{}", json_path, filename);
            fs::rename(oldname, newname).expect("重命名(移动)文件出错");
        }

        if !message.is_empty() {
            let temp = send_xfx(message).await;
            println!("发送消息结果：{:?}", temp);
        }
    } else if coin_type.eq("chia") {
        println!("处理奇亚爆块信息:去官方区块浏览器取得爆块信息");
        /* 先取得此类矿池 */
        let mut message = String::new(); //定义要发送的消息提醒
        let pool_data = get_type_pool("chia", &connection);
        for pool in pool_data {
            let pool_id = pool.0; //矿池ID
            let pool_name = pool.1.unwrap_or_else(|| String::new()); //矿池名
                                                                     // let pool_type = pool.2.unwrap_or_else(|| String::new()); //矿池类型
            let is_allot = pool.3; //矿池是否自动分币

            /* 取得矿池后，还要去取得矿工号(矿池的钱包) */
            let groover_data = get_pool_groover(pool_id, &connection);
            if (groover_data.is_empty()) {
                println!("查无矿工号，跳过……");
                continue;
            }

            for groover in groover_data {
                // let wallet_id = groover.6.expect("钱包ID没填");   //钱包ID，在奇亚这个币里暂时不用
                let wallet = groover.7.expect("无钱包地址");
                let pool_order = groover.4.unwrap();
                let groover_id = groover.0;
                let groover_name = groover.5.unwrap_or_else(|| {
                    let temp = format!("{}号矿工", pool_order);
                    return temp;
                });

                /* 从区块浏览器中找到爆块信息 */
                get_chiaexplorer(
                    pool_id,
                    &pool_name,
                    pool_order,
                    groover_id,
                    &groover_name,
                    &wallet,
                    is_allot,
                    &connection,
                )
                .await;
            }
        }
        // if !message.is_empty() {
        //     let temp = send_xch(message).await; //测试时先取沙下发送钉钉消息
        //     println!("发送消息结果：{:?}", temp);
        // }
    } else if coin_type.eq("cru") {
        let two_arg = std::env::args().nth(2).unwrap_or("logic".to_string());
        if two_arg.eq("logic") {
            println!("CRU币扫块");
            let _susses = logic(&connection).await;
        } else if two_arg.eq("record") {
            // println!("每隔6小时记录所有CRU矿池和矿机算力");
            cru_record::cru_record(&connection).await;
        } else {
            println!("这里处理CRU矿机监控");
            // cru_server(&connection);
            cru_server_new(&connection).await;
        }
    } else if coin_type.eq("test") {
        test_u64();
        // test_file_lock();
    }

    //下面单单只处理MASS的爆块
    // let pool_data = get_pool_wallet("mass", &connection);
    // for wallet_temp in pool_data {
    //     let wallet_id = wallet_temp.2.expect("钱包ID没填"); //钱包ID
    //     let wallet = wallet_temp.3.expect("钱包地址没填"); //钱包地址
    //     let pool_name = wallet_temp.1.expect("矿池名不能为空");
    //     let pool_id = wallet_temp.0; //矿池ID
    //     let is_allot = wallet_temp.4; //是否自动分币：0 默认不自动，1自动分币

    //     command_find_block_histories(
    //         pool_id,
    //         &pool_name,
    //         &wallet_id,
    //         &wallet,
    //         is_allot,
    //         &connection,
    //     );
    // }

    // println!("发送钉钉消息:{}", message);
    println!("执行成功");
}

async fn devide_type(connection: &MysqlPooledConnection) {
    let pool_data = get_pool(&connection);
    for pool in pool_data {
        let pool_id = pool.0; //矿池ID
        let pool_name = pool.1.unwrap_or_else(|| String::new()); //矿池名
        let pool_type = pool.2.unwrap_or_else(|| String::new()); //矿池类型
        let is_allot = pool.3; //矿池是否自动分币

        /* 添加非mass类的矿池跳过下面的执行 */
        if !pool_type.eq("mass") {
            println!("非mass类矿池，跳过……");
            continue;
        }

        /* 取得矿池后，还要去取得矿工号(矿池的钱包) */
        let groover_data = get_pool_groover(pool_id, connection);
        if (groover_data.is_empty()) {
            println!("查无矿工号，跳过……");
            continue;
        }

        for groover in groover_data {
            let wallet_id = groover.6.expect("钱包ID没填");
            let wallet = groover.7.expect("无钱包地址");
            let pool_order = groover.4.unwrap();
            let groover_id = groover.0;
            let groover_name = groover.5.unwrap_or_else(|| {
                let temp = format!("{}号矿工", pool_order);
                return temp;
            });

            // println!("执行进来=========start=====");

            command_find_block_histories(
                pool_id,
                &pool_name,
                groover_id,
                &groover_name,
                &wallet_id,
                &wallet,
                is_allot,
                connection,
            )
            .await;
            // println!("执行进来=========end=====");
        }
    }
}
