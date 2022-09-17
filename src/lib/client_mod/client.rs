use std::collections::{HashMap, HashSet};

enum Tx {
    Withdraw(f64),
    Deposit(f64)
}


struct Client{
    client: u16,
    available: f64,
    held: f64,
    total: f64,
    locked: f64,
    txs: HashMap<u32, Tx>,
    disputed_txs: HashSet<u32>
}




