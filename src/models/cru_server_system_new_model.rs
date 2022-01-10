use crate::db::MysqlPooledConnection;
use crate::schema::cru_server_system_new;
use crate::schema::cru_server_system_new::dsl::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Insertable)]
#[table_name = "cru_server_system_new"]
pub struct AddCruServerSystemNew {
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
    pub update_time: Option<NaiveDateTime>,
}

impl AddCruServerSystemNew {
    pub fn insert(&self, connection: &MysqlPooledConnection) -> usize {
        let rows = diesel::insert_into(cru_server_system_new)
            .values(self)
            .execute(connection)
            .expect("cru_server_system_new表插入数据出错");
        rows
    }

    pub fn update(&self, pkey: i64, connection: &MysqlPooledConnection) -> usize {
        let updated_row = diesel::update(cru_server_system_new.find(pkey))
            .set((
                customer_id.eq(self.customer_id),
                group_owner.eq(self.group_owner.as_ref()),
                account_id.eq(self.account_id.as_ref()),
                cap.eq(self.cap.as_ref()),
                used.eq(self.used.as_ref()),
                spare.eq(self.spare.as_ref()),
                report_slot.eq(self.report_slot),
                reported_files_size.eq(self.reported_files_size.as_ref()),
                generated_at.eq(self.generated_at.as_ref()),
                // srd_complete.eq(self.srd_complete),
                // srd_remaining_task.eq(self.srd_remaining_task),
                // disk_available_for_srd.eq(self.disk_available_for_srd),
                // disk_available.eq(self.disk_available),
                // disk_volume.eq(self.disk_volume),
                // sys_disk_available.eq(self.sys_disk_available),
                // disk_count.eq(self.disk_count),
                chain_status.eq(self.chain_status.as_ref()),
                api_status.eq(self.api_status.as_ref()),
                sworker_status.eq(self.sworker_status.as_ref()),
                smanager_status.eq(self.smanager_status.as_ref()),
                ipfs_status.eq(self.ipfs_status.as_ref()),
                update_time.eq(self.update_time.unwrap()),
            ))
            .execute(connection)
            .expect("更新表cru_server_system_new数据出错");
        updated_row
    }
}

#[derive(Debug, Clone, Queryable)]
pub struct CruServerSystemNewData {
    pub id: i64,
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
    pub update_time: NaiveDateTime,
}

pub fn get_account_id(
    accountid: &String,
    connection: &MysqlPooledConnection,
) -> Option<CruServerSystemNewData> {
    let query = cru_server_system_new.filter(account_id.eq(accountid));
    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    // println!("Sql:{:?}", sql);

    let result = query.first::<CruServerSystemNewData>(connection);
    // Err(diesel::NotFound)
    match result {
        Ok(data) => return Some(data),
        Err(diesel::NotFound) => return None, //查无数据
        Err(error) => {
            println!("查找数据库出现意外错误cru_server_system_new：{:#?}", error);
            return None;
        }
    }
}

pub fn get_ip(ip: &String, connection: &MysqlPooledConnection) -> Option<CruServerSystemNewData> {
    let query = cru_server_system_new.filter(ip1.eq(ip));
    let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("Sql:{:?}", sql);

    let result = query.first::<CruServerSystemNewData>(connection);
    // Err(diesel::NotFound)
    match result {
        Ok(data) => return Some(data),
        Err(diesel::NotFound) => return None, //查无数据
        Err(error) => {
            println!("查找数据库出现意外错误cru_server_system_new：{:#?}", error);
            return None;
        }
    }
}
