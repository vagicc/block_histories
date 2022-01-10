use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct CRUWork {
    pub files: Map<String, Value>,
    pub srd: SRD,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SRD {
    pub srd_complete: i64, //已封装,    主要关注    要比较，相等或者比以前的多，不然就是异常
    pub srd_remaining_task: i64, //剩余工作量  主要关注
    pub disk_available_for_srd: i64, //总工作量
    pub disk_available: i64, //硬盘可用
    pub disk_volume: i64,  //硬盘剩余量  ，跟昨天最后一次对比  ,记录每天最后一次
    pub sys_disk_available: i64, //硬盘已使用
    pub srd_detail: HashMap<String, DISKS>, //每个硬盘详情，统计有几个，用来统计。   每次比较不能比上次少。一般来说是相同的数量。不然就是异常
}

#[derive(Debug, Clone, Deserialize)]
pub struct DISKS {
    pub srd: i64,
    pub srd_avail: i64,
    pub avail: i64,
    pub volumn: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GroupMembersDataCheck {
    pub code: i8,
    pub message: String,
    pub generated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GroupMembersData {
    pub code: i8,
    pub message: String,
    pub generated_at: i64,
    pub data: MembersData, //这个data有时有有时没呢？完全不知道怎么处
                           // pub data:Option<MembersData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MembersData {
    pub count: i32,
    pub list: Option<Vec<Member>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Member {
    pub account_id: String,
    pub cap: String,
    pub used: String,
    pub spare: String,
    pub report_slot: i64,
    pub reported_files_size: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GroupOwnerData {
    pub code: i8,
    pub message: String,
    pub generated_at: i64,
    pub data: PoolCapData,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PoolCapData {
    pub stash: String,
    pub controller: String,
    pub power: String,
    pub limit_stake: String,
    pub total_stake: String,
    pub members: i32,
    pub cap: String,
}
