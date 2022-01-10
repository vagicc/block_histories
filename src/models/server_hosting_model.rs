use crate::db::MysqlPooledConnection;
use crate::schema::server_hosting;
use crate::schema::server_hosting::dsl::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::QueryDsl;

//添加 `account_id` varchar(150) DEFAULT NULL COMMENT '矿机地址',
// ALTER TABLE coin.server_hosting ADD account_id varchar(150) DEFAULT NULL COMMENT '矿机地址';

/* 只做查询数据库返回的结构体示例 */
#[derive(Debug, Clone, Queryable)]
pub struct ServerHostingData {
    pub id: i64,
    pub server_name: String,
    pub computer_room: Option<String>,
    pub cpuinfo: Option<String>,
    pub cpu_nums: Option<i8>,
    pub hard_disk: Option<String>,
    pub memory: Option<String>,
    pub radiator: Option<String>,
    pub power: Option<String>,
    pub ip1: Option<String>,
    pub ip2: Option<String>,
    pub state: bool,
    pub notes: Option<String>,
    pub coin_type: Option<String>,
    pub admin_id: Option<i64>,
    pub customer_id: u32,
    pub create_time: Option<NaiveDateTime>,
    pub account_id: Option<String>,
}

pub fn get_coin_type(cointype: &str, connection: &MysqlPooledConnection) -> Vec<ServerHostingData> {
    let query = server_hosting.filter(coin_type.eq(cointype));
    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("SQL:{:?}", sql);

    let default: Vec<ServerHostingData> = Vec::new();
    let data = query
        .get_results::<ServerHostingData>(connection)
        .unwrap_or(default);
    data
}
