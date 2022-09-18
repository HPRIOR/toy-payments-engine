use std::collections::HashMap;

use crate::client_mod::client::Client;
use crate::io_mod::csv_io::{TxRow, TxType};

pub struct CsvProcessor {
    clients: HashMap<u16, Client>,
}

impl CsvProcessor {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn process_rows(&mut self, rows: &Vec<TxRow>) {
        rows.iter().for_each(|row| self.process_row(row))
    }

    fn process_row(&mut self, row: &TxRow) {
        match row.tx_type {
            // assuming deposit and withdrawal rows always contain an amount
            TxType::Deposit => self.client_call(&|c| c.deposit(row.tx, row.amount.unwrap()), row),
            TxType::Withdrawal => {
                self.client_call(&|c| c.withdraw(row.tx, row.amount.unwrap()), row)
            }
            TxType::Dispute => self.client_call(&|c| c.dispute(row.tx), row),
            TxType::Resolve => self.client_call(&|c| c.resolve(row.tx), row),
            TxType::ChargeBack => self.client_call(&|c| c.chargeback(row.tx), row),
        }
    }

    /// Handles the creation of new clients and delegates client method call to function pointer
    fn client_call(&mut self, client_cmd: &dyn Fn(&mut Client), row: &TxRow) {
        let maybe_client = self.clients.get_mut(&row.client);
        match maybe_client {
            Some(client) => client_cmd(client),
            None => match row.tx_type {
                // new clients can only be created by withdraws and desposits
                TxType::Deposit | TxType::Withdrawal => {
                    let mut c = Client::new(row.client);
                    client_cmd(&mut c);
                    self.clients.insert(row.client, c);
                }
                _ => {}
            },
        }
    }

    pub fn client_results(&self) -> Vec<&Client> {
        self.clients.iter().map(|(_, c)| c).collect()
    }
}
