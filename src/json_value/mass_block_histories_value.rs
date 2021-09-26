use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WalletData {
    pub chainId: String,
    pub walletId: String,
    // pub type:u8,
    pub totalBalance: String,
    pub externalKeyCount: i32,
}

#[derive(Debug, Deserialize)]
pub struct MassBlockHistories {
    pub histories: Vec<Histories>,
}

#[derive(Debug, Deserialize,Clone)]
pub struct Histories {
    pub txId: String,
    pub blockHeight: String,
    pub outputs: Vec<Output>,
    pub fromAddresses: Vec<String>,
}

#[derive(Debug, Deserialize,Clone)]
pub struct Output {
    pub address: String,
    pub amount: String,
}

/* 区块高度查到的详情结构体 */
#[derive(Debug, Deserialize)]
pub struct BlockHeight {
    pub hash: String,
    pub chainId: String,
    pub version: String,
    pub height: String,
    pub confirmations: String,
    pub time: String,
    pub previousHash: String,
    //  下面还有，只解析这么点就行
}
