use crate::db::MysqlPooledConnection;
use crate::schema::cru_server_system;
use crate::schema::cru_server_system::dsl::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Insertable)]
#[table_name = "cru_server_system"]
pub struct AddCruServerSystem {
    pub server_id: u32,
    pub customer_id: i64,
    pub ip1: Option<String>,
    pub ip2: Option<String>,

    pub group_owner: Option<String>,
    pub account_id: Option<String>,
    pub cap: Option<String>,
    pub used: Option<String>,
    pub spare: Option<String>,
    pub report_slot: i64,
    pub reported_files_size: Option<String>,
    pub generated_at: Option<i64>,

    // pub srd_complete: i64,
    // pub srd_remaining_task: i64,
    // pub disk_available_for_srd: i64,
    // pub disk_available: i64,
    // pub disk_volume: i64,
    // pub sys_disk_available: i64,
    // pub disk_count: i32,
    pub chain_status: Option<String>,
    pub api_status: Option<String>,
    pub sworker_status: Option<String>,
    pub smanager_status: Option<String>,
    pub ipfs_status: Option<String>,
    pub create_time: Option<NaiveDateTime>,
}

impl AddCruServerSystem {
    pub fn insert(&self, connection: &MysqlPooledConnection) -> usize {
        let rows = diesel::insert_into(cru_server_system)
            .values(self)
            .execute(connection)
            .expect("cru_server_system表插入数据出错");
        rows
    }
}
