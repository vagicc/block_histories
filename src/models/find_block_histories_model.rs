use crate::db::MysqlPooledConnection;
use crate::schema::find_block_histories;
use crate::schema::find_block_histories::dsl::*;
use diesel::prelude::*;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::QueryDsl;

#[derive(Debug, Insertable, Clone, Queryable, PartialEq, Eq)]
#[table_name = "find_block_histories"]
pub struct FindBlockHistories {
    pub id: i64,
    pub coin_type: Option<String>, //币种
    pub block_num: Option<i64>,    //开始区块
    pub cycles_num: Option<i64>, //上个任务的周期查找区块量,block_num-cycles_num=上一条任务开始区块
    pub status: i8,              //任务状态：0没有任务在执行,1有任务在执行，2执行完成
    pub optimism: i64,           //用于任务调度乐观锁
    pub modify: NaiveDateTime,
    pub create_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "find_block_histories"]
pub struct NewFindBlockHistories {
    pub coin_type: Option<String>, //币种
    pub block_num: i64,            //开始区块
    pub cycles_num: i64, //上个任务的周期查找区块量,block_num-cycles_num=上一条任务开始区块
    pub status: i8,      //任务状态：0没有任务在执行,1有任务在执行，2执行完成
    pub optimism: Option<i64>, //用于任务调度乐观锁
    pub modify: Option<NaiveDateTime>,
    pub create_time: Option<NaiveDateTime>,
}
impl NewFindBlockHistories {
    pub fn insert(&self, connection: &MysqlPooledConnection) -> usize {
        let rows = diesel::insert_into(find_block_histories)
            .values(self)
            .execute(connection)
            .expect("find_block_histories表插入数据出错");
        rows
    }
}

pub fn get_start_block(
    cointype: &str,
    connection: &MysqlPooledConnection,
) -> Option<FindBlockHistories> {
    let query = find_block_histories
        .filter(coin_type.eq(cointype))
        .filter(status.eq(0));

    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("取得开始区块的SQL:{:?}", sql);

    let data = query
        .get_results::<FindBlockHistories>(connection)
        .expect("出错了，没区块可执行");

    let mut return_data: Option<FindBlockHistories> = None;

    for fblock in data {
        let updated_row = set_status(fblock.id, 1, fblock.optimism, connection);
        if updated_row > 0 {
            return_data = Some(fblock);
            break;
        }
    }
    return_data
}

/* 更新区块任务正在执行 */
pub fn set_status(
    pkey: i64,
    new_status: i8,
    old_optimism: i64,
    connection: &MysqlPooledConnection,
) -> usize {
    let query = diesel::update(
        find_block_histories
            .filter(id.eq(pkey))
            .filter(optimism.eq(old_optimism)),
    )
    .set((status.eq(new_status), optimism.eq(old_optimism + 1)));

    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("更新任务调度SQL:{:?}", sql);

    let updated_row = query
        .execute(connection)
        .expect("更新表find_block_histories数据出错,这里应该不中断");
    updated_row
}
