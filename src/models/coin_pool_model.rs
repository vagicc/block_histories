use crate::db::MysqlPooledConnection;
use crate::schema::coin_pool;
use crate::schema::coin_pool::dsl::*;
use diesel::dsl::sum;
use diesel::prelude::*;
// use diesel::result::Error;

pub fn get_pool(
    connection: &MysqlPooledConnection,
) -> Vec<(i64, Option<String>, Option<String>, i8)> {
    //要返回：矿池ID、矿池名、矿池类型、是否自动分币
    let query = coin_pool.select((id, pool_name, pool_type, allot));

    // let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("SQL:{:?}", sql);

    let data = query
        .get_results::<(i64, Option<String>, Option<String>, i8)>(connection)
        .expect("池池表无数据");
    data
}

/* 取得一个类型的矿池 */
pub fn get_type_pool(
    pooltype: &str,
    connection: &MysqlPooledConnection,
) -> Vec<(i64, Option<String>, Option<String>, i8)> {
    let query = coin_pool
        .select((id, pool_name, pool_type, allot))
        .filter(pool_type.eq(pooltype));

    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("取得一类矿池的SQL:{:?}", sql);

    let data = query
        .get_results::<(i64, Option<String>, Option<String>, i8)>(connection)
        .expect("取得矿池出错");
    data
}

pub fn get_pool_wallet(
    pooltype: &str,
    connection: &MysqlPooledConnection,
) -> Vec<(i64, Option<String>, Option<String>, Option<String>, i8)> {
    let query = coin_pool
        .select((id, pool_name, wallet_id, wallet_address, allot))
        // .select((id, pool_name))
        .filter(pool_type.eq(pooltype));

    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("SQL:{:?}", sql);

    // let data = query
    //     .get_results::<(i64, String, String)>(connection)
    //     .unwrap();

    // let data = query
    //     .first::<(i64, Option<String>)>(connection)
    //     .unwrap_or_else(|error| {
    //         match error {
    //             Error::NotFound => return (0,None), //找不到数据直接返回（0,0）
    //             _ => panic!("数据查询失败: {:#?}", error),
    //         }
    //         // panic!("数据查询失败: {:#?}", error);
    //     });

    // let data = query
    //     .get_results::<(i64, Option<String>)>(connection)
    //     .unwrap();

    let data = query
        .get_results::<(i64, Option<String>, Option<String>, Option<String>, i8)>(connection)
        .unwrap();

    data
}

/* 取得矿池总算力 */
pub fn get_spaces_count(pool_id: i64, connection: &MysqlPooledConnection) -> i64 {
    let query = coin_pool.select(spaces_count).find(pool_id);
    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("取得矿池总算力SQL:{}", sql);
    let spaces = query.first::<i64>(connection).expect("取得矿池总算力出错");
    spaces
}

/* 取得此类矿池总算力== sum的结果集 出错，不知道是什么返回类型 */
pub fn _error_get_spaces_count(pooltype: &str, connection: &MysqlPooledConnection) {
    let query = coin_pool
        .select(sum(spaces_count))
        .filter(pool_type.eq(pooltype));

    // let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("取得MASS类型矿池总算力SQL:{:?}", sql);

    // let data = query.first::<i64>(connection);
    // let asd = query.first::<(Ok(Option<i64>))>(connection);
    // let asd = query.first::<(Ok(i64))>(connection);
    // let data = query.first::<Option<i64>,_>(connection);
    // let teka=Ok(Some(12i64)); // Result<Option<i64>, Error>

    //assert_eq!(Ok(Some(12i64)), animals.select(sum(legs)).first(&connection));
}
