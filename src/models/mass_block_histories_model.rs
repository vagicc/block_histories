use crate::schema::mass_block_histories;
use crate::schema::mass_block_histories::dsl::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Debug, Insertable, Clone)]
#[table_name = "mass_block_histories"]
pub struct AddMassBlockHistories {
    pub tx_id: String,
    pub block_height: i64,
    pub block_time: i64,
    pub amount: BigDecimal,
    pub from_addresses: String,
    pub wallet_address: String,
    pub pool_id: Option<i64>,
    pub pool_name: Option<String>,
    pub pool_type: Option<String>,
    pub pool_address: Option<String>,
    pub groover_id: Option<i64>,
    pub groover_name: Option<String>,
    pub status: i8,
    pub modify: Option<NaiveDateTime>,
}

impl AddMassBlockHistories {
    pub fn insert(&self, connection: &MysqlConnection) -> usize {
        let rows = diesel::insert_into(mass_block_histories)
            .values(self)
            .execute(connection)
            .expect("mass_block_histories表插入数据出错");
        rows
    }
}

pub fn insert_array(data: Vec<&AddMassBlockHistories>, connection: &MysqlConnection) {
    let rows = diesel::insert_into(mass_block_histories)
        .values(data)
        .execute(connection)
        .expect("mass_block_histories表插入数据出错");
}

pub fn get_tx_id(txid: &String, connection: &MysqlConnection) -> (i64, i64) {
    let query = mass_block_histories
        .select((id, pool_id))
        .filter(tx_id.eq(txid));

    // let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("SQL:{:?}", sql);

    let data = query
        .first::<(i64, i64)>(connection)
        .unwrap_or_else(|error| match error {
            Error::NotFound => return (0, 0),
            _ => panic!("数据查询失败: {:#?}", error),
        });

    data
}

pub fn update_pool(key: i64, data: (i64, &String, &str), connection: &MysqlConnection) {
    let updaded_row = diesel::update(mass_block_histories.find(key))
        .set((
            pool_id.eq(data.0),
            pool_name.eq(data.1),
            pool_type.eq(data.2),
        ))
        .execute(connection)
        .unwrap();
}

pub fn get_day_count_old(poolid: i64, connection: &MysqlConnection) -> i64 {
    let query = mass_block_histories.select(pool_id.eq(poolid)).count();
    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("SQL:{:?}", sql);
    let count: i64 = query.get_result(connection).expect("出错");
    count
}

// 统计矿工今日爆块
pub fn get_groover_day_count(grooverid: i64, connection: &MysqlConnection) -> i64 {
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

    let query = mass_block_histories
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

pub fn get_day_count(poolid: i64, connection: &MysqlConnection) -> i64 {
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

    let query = mass_block_histories
        .select(id)
        .filter(pool_id.eq(poolid)) //这行刚添加，还没更新到生产
        .filter(block_time.ge(zero_morning_time)) // ge Creates a SQL `>=` expression.
        .filter(block_time.le(now_time)) // Creates a SQL `<=` expression.
        .count();

    // let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("统计今日爆块SQL:{:?}", sql);

    let count: i64 = query.get_result(connection).expect("出错");
    count
}

pub fn get_day_count_all(connection: &MysqlConnection) -> i64 {
    /* 取得时间戳处理 */
    use chrono::prelude::Local;
    use chrono::{Datelike, TimeZone};
    let now = Local::now();
    let now_time = now.timestamp(); //现在的时间 戳
    let today_str = now.format("%Y-%m-%d").to_string();
    let today_o = format!("{}-{}-{} 00:00:00", now.year(), now.month(), now.day());
    // println!("今天字符串：{}", today_o);
    let zero_morning = Local
        .datetime_from_str(&today_o.to_owned(), "%Y-%m-%d %H:%M:%S")
        .unwrap();
    let zero_morning_time = zero_morning.timestamp(); //转成时间戳
                                                      // println!("今天零晨的时间戳：{}", zero_morning_time);

    let query = mass_block_histories
        .select(id)
        .filter(block_time.ge(zero_morning_time)) // ge Creates a SQL `>=` expression.
        .filter(block_time.le(now_time)) // Creates a SQL `<=` expression.
        .count();

    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("统计今日爆块SQL:{:?}", sql);

    let count: i64 = query.get_result(connection).expect("出错");
    count
}

pub fn get_pool_count(poolid: i64, connection: &MysqlConnection) -> i64 {
    // let query = mass_block_histories.select(pool_id.eq(poolid)).count();
    let query = mass_block_histories
        .select(id)
        .filter(pool_id.eq(poolid))
        .count();
    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("SQL:{:?}", sql);
    let count: i64 = query.get_result(connection).expect("出错");
    count
}

pub fn get_all_count(connection: &MysqlConnection) -> i64 {
    // let query = mass_block_histories.select(pool_id.eq(poolid)).count();
    let query = mass_block_histories.select(id).count();
    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("SQL:{:?}", sql);
    let count: i64 = query.get_result(connection).expect("出错");
    count
}

pub fn time_test() {
    println!("==============时间转换为时间戳，时间戳转换为时间=====================");
    use chrono::prelude::Local;
    use chrono::{Datelike, TimeZone};
    let now = Local::now();
    let today_str = now.format("%Y-%m-%d").to_string();
    let today_o = format!("{}-{}-{} 00:00:00", now.year(), now.month(), now.day());
    println!("今天字符串：{}", today_o);
    let zero_morning = Local
        .datetime_from_str(&today_o.to_owned(), "%Y-%m-%d %H:%M:%S")
        .unwrap();
    let zero_morning_time = zero_morning.timestamp(); //转成时间戳
    println!("今天零晨的时间戳：{}", zero_morning_time);

    let zero_date = Local.timestamp(zero_morning_time, 0); //时间戳转换为时间
    println!(
        "今天零晨的时间戳转换成时间：{}",
        zero_date.format("%Y-%m-%d %H:%M:%S")
    );
    println!("==============时间转换为时间戳，时间戳转换为时间=====================");
}
