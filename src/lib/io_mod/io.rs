use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TxRow {
    #[serde(alias = "type")]
    pub tx_type: TxType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub enum TxType {
    #[serde(alias = "deposit")]
    Deposit,
    #[serde(alias = "withdrawal")]
    Withdrawal,
    #[serde(alias = "dispute")]
    Dispute,
    #[serde(alias = "resolve")]
    Resolve,
    #[serde(alias = "chargeback")]
    ChargeBack,
}
