use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Clone)]
pub struct XFXDATA {
    pub data: Vec<XFXWallet>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct XFXWallet {
    pub netBalanceAmoutText: String,
    pub netBalanceAmout: String,
    pub netBalanceAmoutMojo: String,
    pub farmedBalanceAmoutText: String,
    pub farmedBalanceAmout: String,
    pub farmedBalanceAmoutMojo: String,
    pub address: String,
    pub record: Vec<XFXRecord>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct XFXRecord {
    pub time_str: String,
    pub time: i64,
    // // pub type: String,
    pub coin: String,
    pub amountText: String,
    pub amount: String,
    pub amountMojo: String,
    pub blockHeight: String,
    pub heightSpent: String,
}
