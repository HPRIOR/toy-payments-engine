use std::ffi::OsString;

use test_utils::create_csv;
use toy_payments_lib::process_payments;

extern crate test_utils;

#[test]
fn basic_example() {
    let sut = process_payments(&OsString::from("tests/resources/basic_example.csv")).unwrap();
    let expected = create_csv(vec![
        ["1", "1.5", "0.0", "1.5", "false"],
        ["2", "2.0", "0.0", "2.0", "false"],
    ]);
    assert_eq!(sut, expected)
}

#[test]
fn cannot_withdraw_over_avail() {
    let sut = process_payments(&OsString::from("tests/resources/withdraw_over_avail.csv")).unwrap();
    let expected = create_csv(vec![["1", "20.0", "0.0", "20.0", "false"]]);
    assert_eq!(sut, expected)
}



// Assumption: withdraws that occur after a dispute can be enacted post-hoc if they are within
// the limits of available funds after resolution
#[test]
fn withdrawals_retroactively_resolved() {
    let sut = process_payments(&OsString::from("tests/resources/retroactive_resolve.csv")).unwrap();
    let expected = create_csv(vec![["1", "0.0", "0.0", "0.0", "false"]]);
    assert_eq!(sut, expected)
}

// Assumptiom: retroactive resolutions of withdrawals only apply to those withdraws rejected
// after a dispute. If a transaction occurs before a dispute and is rejected, then it is based on
// the available funds at the time of the transaction, and only retroactively
// dependant on the disputes occuring prior to it
#[test]
fn no_retroactive_resolve_for_withdraw_prior_to_dispute() {
    let sut = process_payments(&OsString::from(
        "tests/resources/retroactive_resolve_with_rejected_withdrawal.csv",
    ))
    .unwrap();
    let expected = create_csv(vec![["1", "100.0", "50.0", "150.0", "false"]]);
    assert_eq!(sut, expected)
}
