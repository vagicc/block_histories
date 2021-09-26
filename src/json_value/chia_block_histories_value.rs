use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Clone)]
pub struct ChiaData {
    pub coins: Vec<XCHCoin>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct XCHCoin {
    pub coin_id: String,
    pub puzzle_hash: String,
    pub parent_coin_info: String,
    pub amount: i64,
    pub coinbase: bool,
    pub confirmed_block_index: i64,
    pub spent: bool,
    pub spent_block_index: i64,
    pub timestamp: String,
    pub block_header_hash: String,
    pub block_height: i64,
}
