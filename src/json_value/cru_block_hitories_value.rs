use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Clone)]
pub struct CRUMetadata {
    pub code: i8,
    pub message: String,
    pub data: CRUMetadataDATA,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CRUMetadataDATA {
    pub blockNum: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CRUExtrinsics {
    pub code: i8,
    pub message: String,
    pub data: CRUExtrinsicsData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CRUExtrinsicsData {
    pub count: i32,
    pub extrinsics: Option<Vec<Extrinsics>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Extrinsics {
    pub block_num: i32,
    pub extrinsic_index: String,
    pub account_id: String,
}

/* 交易详情 */
#[derive(Debug, Clone, Deserialize)]
pub struct CRUExtrinsic {
    pub code: i8,
    pub message: String,
    pub generated_at: i64,
    pub data: ExtrinsicData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtrinsicData {
    pub block_timestamp: i64,    //出块时间戳
    pub block_num: i64,          //区块数
    pub extrinsic_index: String, //交易号
    pub account_id: String,      //钱包，账户ID
    pub signature: String,
    pub extrinsic_hash: String, //交易hash,具有唯一性
    pub success: bool,
    pub event: Vec<EventData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventData {
    pub event_index: String, //与extrinsic_index交易号相同
    pub block_num: i64,      //区块数
    pub extrinsic_idx: i32,
    pub module_id: String,
    pub event_id: String,
    pub params: String, //这个还要进一步解析成json
    // pub params: Vec<ParamData>,
    pub event_idx: i32,
    pub extrinsic_hash: String,
    pub finalized: bool,
    pub block_timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParamData {
    pub r#type: String,
    pub value: String,
}
