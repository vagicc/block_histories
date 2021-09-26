use crate::common::get_env;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type MysqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn _mysql_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let database_url = get_env("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::new(manager).expect("msg");
    pool
}

pub fn establish_connection() -> MysqlPooledConnection {
    let database_url = get_env("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    // let pool=Pool::new(manager).expect("msg");
    // let conn=pool.get().expect("msg");

    let pool = Pool::builder()
        .max_size(1)
        .test_on_check_out(true)
        .build(manager)
        .unwrap();
    let conn = pool.get().unwrap();

    conn
}

// pub fn _my_mysql_connection() -> MysqlConnection {
//     let database_url = get_env("DATABASE_URL").to_owned();  //这里有错误
//     let conn = MysqlConnection::establish(database_url).expect("kk");
//     conn
// }
