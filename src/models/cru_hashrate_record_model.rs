use crate::db::MysqlPooledConnection;
use crate::schema::cru_hashrate_record;
use crate::schema::cru_hashrate_record::dsl::*;
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;

#[derive(Debug, Clone, Insertable)]
#[table_name = "cru_hashrate_record"]
pub struct AddcruHashrateRecord {
    pub daily: NaiveDate,
    pub hours: i8,

    pub pool_id: i64,
    pub pool_name: Option<String>,
    pub pool_type: Option<String>,
    pub groover_id: i64,
    pub pool_order: Option<i32>,
    pub groover: Option<String>,
    pub wallet_id: Option<String>,
    pub wallet_address: Option<String>,
    pub status: i8,

    pub pool_power: Option<String>,
    pub pool_limit_stake: Option<String>,
    pub pool_total_stake: Option<String>,
    pub pool_members: Option<i32>,
    pub pool_cap: Option<String>,
    pub server_id: u32,
    pub customer_id: i64,
    pub account_id: Option<String>,
    pub cap: Option<String>,
    pub used: Option<String>,
    pub spare: Option<String>,
    pub report_slot: i64,
    pub reported_files_size: Option<String>,
    pub generated_at: Option<i64>,
    pub update_time: NaiveDateTime,
}

impl AddcruHashrateRecord {
    pub fn insert(&self, connection: &MysqlPooledConnection) -> usize {
        let insert_id = diesel::insert_into(cru_hashrate_record)
            .values(self)
            .execute(connection)
            .expect("cru_hashrate_record表插入数据出错");
        insert_id
    }
}
