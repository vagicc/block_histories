use crate::db::MysqlPooledConnection;
use crate::schema::chia_block_histories;
use crate::schema::chia_block_histories::dsl::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Debug, Insertable, Clone)]
#[table_name = "chia_block_histories"]
pub struct AddChiaBlockHistories {
    pub coin_id: String,
    pub puzzle_hash: Option<String>,
    pub parent_coin_info: Option<String>,
    pub amount: Option<i64>,
    pub coinbase: Option<String>,
    pub spent: Option<String>,
    pub spent_block_index: Option<i64>,
    pub timestamp: Option<i64>,
    pub block_header_hash: Option<String>,
    pub block_height: Option<i64>,

    pub pool_id: i64,
    pub pool_name: Option<String>,
    pub pool_type: Option<String>,
    pub groover_id: i64,
    pub pool_order: Option<i32>,
    pub groover: Option<String>,
    pub wallet_id: Option<String>,
    pub wallet_address: Option<String>,
    pub status: i8,
    pub modify: Option<NaiveDateTime>,
    pub create_time: Option<NaiveDateTime>,
}

impl AddChiaBlockHistories {
    pub fn insert(&self, connection: &MysqlPooledConnection) -> usize {
        let rows = diesel::insert_into(chia_block_histories)
            .values(self)
            .execute(connection)
            .expect("chia_block_histories表插入数据出错");
        rows
    }
}

pub fn get_coin_id(coinid: &String, connection: &MysqlPooledConnection) -> i64 {
    let query = chia_block_histories.select(id).filter(coin_id.eq(coinid));
    // let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("SQL:{:?}", sql);
    let data = query
        .first::<i64>(connection)
        .unwrap_or_else(|op| match op {
            diesel::result::Error::NotFound => return 0,
            _ => panic!("查找CHIA爆块表出错"),
        });
    data
}

pub fn set_status(
    coinid: &String,
    new_status: i8,
    modify_time: NaiveDateTime,
    connection: &MysqlPooledConnection,
) -> usize {
    let updated_row = diesel::update(chia_block_histories.filter(coin_id.eq(coinid)))
        .set((status.eq(new_status), modify.eq(modify_time)))
        .execute(connection)
        .expect("更新chia爆块状态为已分币出错");
    updated_row
}

/* 检查区块是否已分币:返回bool类型，true为未分币，false为已分币或者无此条记录的区块 */
pub fn check_allot(coinid: &String, connection: &MysqlPooledConnection) -> bool {
    let mut check = false;
    let query = chia_block_histories
        .select(status)
        .filter(coin_id.eq(coinid));
    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("SQL:{:?}", sql);

    let data = query.first::<i8>(connection).unwrap_or_else(|op| match op {
        diesel::result::Error::NotFound => return 2,
        _ => panic!("查找区块是否已分币出错"),
    });

    if data == 0 {
        check = true;
    }

    check
}

// 统计矿工今日爆块
pub fn get_groover_day_count(grooverid: i64, connection: &MysqlPooledConnection) -> i64 {
    /* 取得时间戳处理 */
    use chrono::prelude::Local;
    use chrono::{Datelike, TimeZone};
    let now = Local::now();
    let now_time = now.timestamp(); //现在的时间 戳
                                    // let today_str = now.format("%Y-%m-%d").to_string();
    let today_o = format!("{}-{}-{} 00:00:00", now.year(), now.month(), now.day());
    // println!("今天字符串：{}", today_o);
    let zero_morning = Local
        .datetime_from_str(&today_o.to_owned(), "%Y-%m-%d %H:%M:%S")
        .unwrap();
    let zero_morning_time = zero_morning.timestamp(); //转成时间戳
                                                      // println!("今天零晨的时间戳：{}", zero_morning_time);

    let query = chia_block_histories
        .select(id)
        .filter(groover_id.eq(grooverid)) //这行刚添加，还没更新到生产
        .filter(timestamp.ge(zero_morning_time)) // ge Creates a SQL `>=` expression.
        .filter(timestamp.le(now_time)) // Creates a SQL `<=` expression.
        .count();

    // let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("统计今日矿工爆块SQL:{:?}", sql);

    let count: i64 = query.get_result(connection).expect("出错");
    count
}
