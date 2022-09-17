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
        if self.locked || self.available < amount {
            return;
        }

        self.total -= amount;
        self.available -= amount;
        self.txs.insert(tx, Tx::Withdraw(amount));
    }

    fn dispute(&mut self, tx: u32) -> () {
        // transactions cannot be disputed more than once
        let is_disputed = self.disputed_txs.contains(&tx);
        if self.locked || is_disputed{
            return;
        }

        let maybe_tx_amount = self.txs.get(&tx);
        // only deposits can be disputed (see readme)
        if let Some(Tx::Deposit(tx_amount)) = maybe_tx_amount {
            self.available -= tx_amount;
            self.held += tx_amount;
            self.disputed_txs.insert(tx);
        }
    }

    fn resolve(&mut self, tx: u32) -> () {

    }
    fn chargeback(&mut self, tx: u32) -> () {
        todo!()
    }

}
