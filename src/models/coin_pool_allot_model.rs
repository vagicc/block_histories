use crate::db::MysqlPooledConnection;
use crate::schema::coin_pool_allot;
use crate::schema::coin_pool_allot::dsl::*;

use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::*, row};

#[derive(Debug, Clone, Insertable)]
#[table_name = "coin_pool_allot"]
pub struct AddCoinPoolAllot {
    pub ratio_id: i64,
    pub daily: NaiveDate,
    pub pool_id: i64,
    pub pool_name: Option<String>,
    pub pool_type: String,
    pub pool_wallet_address: Option<String>,
    pub spaces_count: i64,
    pub customer_id: i64,
    pub username: Option<String>,
    pub customer_wallet_address: Option<String>,
    pub spaces: i64,
    pub ratio: BigDecimal,
    pub coin_number: BigDecimal,
    pub amount: BigDecimal,
    pub block_height: Option<i64>,
    pub block_time: Option<i64>,
    pub status: i8,
    pub admin_id: Option<i64>,
    pub create_time: Option<NaiveDateTime>,
    pub last_time: Option<NaiveDateTime>,
}

impl AddCoinPoolAllot {
    pub fn insert(&self, connection: &MysqlPooledConnection) -> usize {
        let rows = diesel::insert_into(coin_pool_allot)
            .values(self)
            .execute(connection)
            .expect("coin_pool_allot表插入数据出错");
        rows
    }
}
