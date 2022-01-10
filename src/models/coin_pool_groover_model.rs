use crate::db::MysqlPooledConnection;
use crate::schema::coin_pool_groover;
use crate::schema::coin_pool_groover::dsl::*;
use diesel::prelude::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::QueryDsl;

#[derive(Debug, Insertable, Clone, Queryable, PartialEq, Eq)]
#[table_name = "coin_pool_groover"]
pub struct CoinPoolGrooverData {
    pub id: i64,
    pub pool_id: i64,
    pub pool_name: Option<String>,
    pub pool_type: Option<String>,
    pub pool_order: Option<i32>,
    pub groover: Option<String>,
    pub wallet_id: Option<String>,
    pub wallet_address: Option<String>,
    pub mnemonic: Option<String>,
    pub passwd: Option<String>,
    pub admin_id: i64,
    pub modify: NaiveDateTime,
    pub create_time: NaiveDateTime,
}

pub fn get_groover(
    address: &String,
    connection: &MysqlPooledConnection,
) -> (
    i64,
    i64,
    Option<String>,
    Option<String>,
    Option<i32>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    let query = coin_pool_groover
        .select((
            id,
            pool_id,
            pool_name,
            pool_type,
            pool_order,
            groover,
            wallet_id,
            wallet_address,
        ))
        .filter(wallet_address.eq(address));

    // let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("取得矿工SQL:{:?}", sql);

    let data = query
        .first::<(
            i64,
            i64,
            Option<String>,
            Option<String>,
            Option<i32>,
            Option<String>,
            Option<String>,
            Option<String>,
        )>(connection)
        .expect("XFX找不到矿工号");

    data
}

/* 取得一个类型的所有矿工 */
pub fn get_pool_type_groover(
    pooltype: &str,
    connection: &MysqlPooledConnection,
) -> Vec<CoinPoolGrooverData> {
    let query = coin_pool_groover.filter(pool_type.eq(pooltype));
    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("取得一类矿工的SQL:{:?}", sql);

    let data = query
        .get_results::<CoinPoolGrooverData>(connection)
        .unwrap_or_else(|op| {
            let temp: Vec<CoinPoolGrooverData> = Vec::new();
            return temp;
        });
    data
}

pub fn get_pool_groover(
    poolid: i64,
    connection: &MysqlPooledConnection,
) -> Vec<(
    i64,
    i64,
    Option<String>,
    Option<String>,
    Option<i32>,
    Option<String>,
    Option<String>,
    Option<String>,
)> {
    let query = coin_pool_groover
        .select((
            id,
            pool_id,
            pool_name,
            pool_type,
            pool_order,
            groover,
            wallet_id,
            wallet_address,
        ))
        .filter(pool_id.eq(poolid));

    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("取得矿工SQL:{:?}", sql);

    let data = query
        .get_results::<(
            i64,
            i64,
            Option<String>,
            Option<String>,
            Option<i32>,
            Option<String>,
            Option<String>,
            Option<String>,
        )>(connection)
        .unwrap_or_else(|op| match op {
            diesel::result::Error::NotFound => return Vec::new(),
            _ => panic!("数据查询失败:{:#?}", op),
        });

    // let data = query
    //     .get_results::<(
    //         i64,
    //         i64,
    //         Option<String>,
    //         Option<String>,
    //         Option<i32>,
    //         Option<String>,
    //         Option<String>,
    //         Option<String>,
    //     )>(connection)
    //     .expect("找不到矿工号");

    /* 处理异常为返回给上一级处理 */
    data
}
