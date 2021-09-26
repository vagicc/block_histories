use crate::db::MysqlPooledConnection;
use crate::schema::xfx_block_histories;
use crate::schema::xfx_block_histories::dsl::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Insertable, Clone)]
#[table_name = "xfx_block_histories"]
pub struct AddXFXBlockHistories {
    pub tx_id: String,
    pub block_height: i64,
    pub height_spent: Option<String>,
    pub block_time: i64,
    pub time_str: Option<String>,
    pub block_type: Option<String>,
    pub amount: Option<BigDecimal>,
    pub amount_text: Option<String>,
    pub amount_mojo: Option<String>,
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

impl AddXFXBlockHistories {
    pub fn insert(&self, connection: &MysqlPooledConnection) -> usize {
        let rows = diesel::insert_into(xfx_block_histories)
            .values(self)
            .execute(connection)
            .expect("xfx_block_histories表插入数据出错");
        rows
    }
}

pub fn get_tx_id(hash: &String, connection: &MysqlPooledConnection) -> i64 {
    let query = xfx_block_histories.select(id).filter(tx_id.eq(hash));

    // let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("SQL:{:?}", sql);

    let data = query
        .first::<i64>(connection)
        .unwrap_or_else(|op| match op {
            diesel::result::Error::NotFound => return 0,
            _ => panic!("查找xfx区块出错"),
        });

    data
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

    let query = xfx_block_histories
        .select(id)
        .filter(groover_id.eq(grooverid)) //这行刚添加，还没更新到生产
        .filter(block_time.ge(zero_morning_time)) // ge Creates a SQL `>=` expression.
        .filter(block_time.le(now_time)) // Creates a SQL `<=` expression.
        .count();

    // let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("统计今日矿工爆块SQL:{:?}", sql);

    let count: i64 = query.get_result(connection).expect("出错");
    count
}
