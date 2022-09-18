#![allow(unused)]
use serde::{Serialize, Serializer};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, PartialOrd, Debug)]
enum Tx {
    Withdraw(f64),
    Deposit(f64),
}

#[derive(Debug)]
struct RejectedTx {
    amount: f64,
    after_disputes: HashSet<u32>,
}

fn fixed_width<S: Serializer>(x: &f64, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&format!("{:.4}", x))
}

#[derive(Debug, Serialize)]
pub struct Client {
    client: u16,
    #[serde(serialize_with = "fixed_width")]
    available: f64,
    #[serde(serialize_with = "fixed_width")]
    held: f64,
    #[serde(serialize_with = "fixed_width")]
    total: f64,
    locked: bool,
    #[serde(skip)]
    txs: HashMap<u32, Tx>,
    #[serde(skip)]
    disputed_txs: HashSet<u32>,
    #[serde(skip)]
    rejected_txs: Vec<RejectedTx>,
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
            rejected_txs: Vec::new(),
        }
    }

    #[allow(unused)]
    fn with_state(id: u16, total: f64, available: f64, held: f64, locked: bool) -> Self {
        Self {
            client: id,
            total,
            available,
            held,
            locked,
            txs: HashMap::new(),
            disputed_txs: HashSet::new(),
            rejected_txs: Vec::new(),
        }
    }

    pub fn deposit(&mut self, tx: u32, amount: f64) {
        if self.locked {
            return;
        }

        self.total += amount;
        self.available += amount;
        self.txs.insert(tx, Tx::Deposit(amount));
    }

    pub fn withdraw(&mut self, tx: u32, amount: f64) {
        if self.locked
            || self.total < amount
            || (self.available < amount && self.disputed_txs.is_empty())
        {
            return;
        }

        if self.available < amount && !self.disputed_txs.is_empty() {
            // keep record of all desputes occuring prior to this transaction
            let curr_open_disputes = self.disputed_txs.clone();
            let rejected_tx = RejectedTx {
                amount,
                after_disputes: curr_open_disputes,
            };
            self.rejected_txs.push(rejected_tx);
            return;
        }

        self.total -= amount;
        self.available -= amount;
        self.txs.insert(tx, Tx::Withdraw(amount));
    }

    pub fn dispute(&mut self, tx: u32) {
        // transactions cannot be disputed more than once
        let is_disputed = self.disputed_txs.contains(&tx);
        if self.locked || is_disputed {
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

    pub fn resolve(&mut self, tx: u32) {
        if self.locked_or_not_disputed(tx) {
            return;
        }
        let maybe_tx_amount = self.txs.get(&tx);
        if let Some(Tx::Deposit(tx_amount)) = maybe_tx_amount {
            self.available += tx_amount;
            self.held -= tx_amount;
            self.resolve_prev_rejected(tx);
        }

        // dispute is resolved
        self.disputed_txs.remove(&tx);
    }

    /// Attempts to resolve rejected tx (withdrawals), that occured after a dispute. 
    fn resolve_prev_rejected(&mut self, resolved_tx: u32) {
        let i_resolved_rej: Vec<usize> = self
            .rejected_txs
            .iter()
            .enumerate()
            .map(|(i, r_tx)| {
                let withdraw_occured_before_resolved_tx = r_tx.after_disputes.contains(&resolved_tx);
                let withdraw_within_avail = r_tx.amount <= self.available;

                if withdraw_occured_before_resolved_tx && withdraw_within_avail {
                    self.available -= r_tx.amount;
                    self.total -= r_tx.amount;
                    return Some(i); 
                };
                None
            })
            .filter_map(|x| x)
            .collect();

        // remove txs which have now been accepted
        // order is reversed to preserve index of subsequent entries
        i_resolved_rej.iter().rev().for_each(|i| {
            self.rejected_txs.remove(*i);
        });
    }

    pub fn chargeback(&mut self, tx: u32) {
        if self.locked_or_not_disputed(tx) {
            return;
        }

        let maybe_tx_amount = self.txs.get(&tx);
        if let Some(Tx::Deposit(tx_amount)) = maybe_tx_amount {
            self.held -= tx_amount;
            self.total -= tx_amount;
            self.locked = true;
        }
    }

    fn locked_or_not_disputed(&self, tx: u32) -> bool {
        let disputed = self.disputed_txs.contains(&tx);
        self.locked || !disputed
    }
}

#[cfg(test)]
mod tests {
    use super::Client;

    #[test]
    fn cannot_withdraw_under_avail() {
        let mut client = Client::with_state(1, 10.0, 5.0, 5.0, false);
        client.withdraw(1, 6.0);
        assert_eq!(client.total, 10.0);
        assert_eq!(client.available, 5.0);
    }
    #[test]
    fn can_withdraw_within_avail() {
        let mut client = Client::with_state(1, 10.0, 5.0, 5.0, false);
        client.withdraw(1, 5.0);
        assert_eq!(client.total, 5.0);
        assert_eq!(client.available, 0.0);
    }

    #[test]
    fn disputed_deposite_reduces_avail() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 5.0);
        client.dispute(1);
        assert_eq!(client.available, 10.0)
    }

    #[test]
    fn disputed_deposite_does_not_reduce_total() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 5.0);
        client.dispute(1);
        assert_eq!(client.total, 15.0)
    }

    #[test]
    fn dispute_will_increase_held_amount() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 5.0);
        client.dispute(1);
        assert_eq!(client.held, 5.0)
    }

    #[test]
    fn disputes_against_withdrawals_are_ignored() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.withdraw(1, 5.0);
        client.dispute(1);
        assert_eq!(client.held, 0.0);
        assert_eq!(client.total, 5.0);
        assert_eq!(client.available, 5.0);
    }

    #[test]
    fn dispute_will_ignore_incorrect_tx() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 5.0);
        client.dispute(2); // no transaction
        assert_eq!(client.total, 15.0)
    }

    #[test]
    fn dispute_is_one_per_tx() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 5.0);
        client.dispute(1);
        client.dispute(1);
        assert_eq!(client.available, 10.0);
    }

    #[test]
    fn resolve_will_release_held_funds() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 5.0);
        client.dispute(1);
        client.resolve(1);

        assert_eq!(client.available, 15.0);
        assert_eq!(client.held, 0.0);
        assert_eq!(client.total, 15.0);
    }

    #[test]
    fn resolve_against_undesputed_tx_is_ignored() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 5.0);
        client.deposit(2, 5.0);
        client.dispute(1);
        client.resolve(2);

        assert_eq!(client.available, 15.0); // reduced by valid dispute
        assert_eq!(client.held, 5.0); // held by valid dispute
        assert_eq!(client.total, 20.0);
    }

    #[test]
    fn resove_against_non_tx_is_ignored() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.dispute(1);
        assert_eq!(client.available, 10.0);
        assert_eq!(client.held, 0.0);
    }

    #[test]
    fn chargeback_locks_account() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 10.0);
        client.dispute(1);
        client.chargeback(1);

        assert!(client.locked)
    }

    #[test]
    fn chargeback_reduces_total() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 10.0);
        assert_eq!(client.total, 20.0);

        client.dispute(1);
        client.chargeback(1);
        assert_eq!(client.total, 10.0)
    }

    #[test]
    fn chargeback_reduces_held() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 10.0);
        client.dispute(1);
        assert_eq!(client.held, 10.0);

        client.chargeback(1);
        assert_eq!(client.held, 0.0)
    }

    #[test]
    fn chargeback_ignored_if_tx_does_not_exist() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 10.0);
        client.dispute(1);

        client.chargeback(2);
        assert_eq!(client.held, 10.0);
        assert_eq!(client.available, 10.0);
        assert_eq!(client.total, 20.0);
        assert!(!client.locked);
    }
    #[test]
    fn chargeback_ignored_if_tx_undesputed() {
        let mut client = Client::with_state(1, 10.0, 10.0, 0.0, false);
        client.deposit(1, 10.0);

        client.chargeback(1);
        assert_eq!(client.held, 0.0);
        assert_eq!(client.available, 20.0);
        assert_eq!(client.total, 20.0);
        assert!(!client.locked);
    }
}
