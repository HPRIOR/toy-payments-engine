use std::{error::Error, ffi::OsString, fs::File};

use serde::Deserialize;

use crate::client_mod::client::Client;

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

pub fn process_csv(csv_path: &OsString) -> Result<Vec<TxRow>, Box<dyn Error>> {
    let file = File::open(csv_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(file);

    let mut rows: Vec<TxRow> = Vec::new();
    for result in rdr.deserialize() {
        let tx_row: TxRow = result?;
        rows.push(tx_row);
    }

    Ok(rows)
}
pub fn output_csv(clients: &Vec<&Client>) -> Result<String, Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(vec![]);

    for client in clients {
        wtr.serialize(client)?
    }

    wtr.flush()?;
    let data = String::from_utf8(wtr.into_inner()?)?;
    Ok(data)
}
