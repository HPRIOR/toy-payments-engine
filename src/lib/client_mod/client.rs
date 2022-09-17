use std::collections::{HashMap, HashSet};

enum Tx {
    Withdraw(f64),
    Deposit(f64),
}

struct Client {
    client: u16,
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
    txs: HashMap<u32, Tx>,
    disputed_txs: HashSet<u32>,
}

impl Client {
    pub fn new(id: u16) -> Self {
        Self {
            client: id,
            total: 0.0,
            available: 0.0,
            held: 0.0,
            locked: false,
            txs: HashMap::new(),
            disputed_txs: HashSet::new(),
        }
    }
    fn deposit(&mut self, tx: u32, amount: f64) -> () {
        if self.locked {
            return;
        }

        self.total += amount;
        self.available += amount;
        self.txs.insert(tx, Tx::Deposit(amount));
    }

    fn withdraw(&mut self, tx: u32, amount: f64) -> () {
        todo!()
    }
    fn dispute(&mut self, tx: u32) -> () {
        todo!()
    }
    fn resolve(&mut self, tx: u32) -> () {
        todo!()
    }
    fn chargeback(&mut self, tx: u32) -> () {
        todo!()
    }
}
