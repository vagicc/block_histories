use crate::db::MysqlPooledConnection;
use crate::schema::coin_pool_ratio;
use crate::schema::coin_pool_ratio::dsl::*;
use diesel::prelude::*;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::QueryDsl;

#[derive(Debug, Insertable, Clone, Queryable, PartialEq, Eq)]
#[table_name = "coin_pool_ratio"]
pub struct AddCoinPoolRatio {
    pub id: i64,
    pub pool_id: Option<i64>,
    pub pool_name: Option<String>,
    pub pool_type: Option<String>,
    pub pool_wallet_address: Option<String>,
    pub customer_id: i64,
    pub username: Option<String>,
    pub customer_wallet_address: Option<String>,
    pub spaces: i64,
    pub ratio: Option<BigDecimal>,
    pub admin_id: i64,
    pub modify: NaiveDateTime,
}

pub fn get_pool_ratio(poolid: i64, connection: &MysqlPooledConnection) -> Vec<AddCoinPoolRatio> {
    let query = coin_pool_ratio.filter(pool_id.eq(3));
    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("取得矿池分币比例SQL:{:?}", sql);

    let data = query
        .load::<AddCoinPoolRatio>(connection)
        .unwrap_or_else(|op| {
            let temp: Vec<AddCoinPoolRatio> = Vec::new();
            return temp;
        });

    data
}
