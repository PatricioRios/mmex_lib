mod common;
use chrono::NaiveDate;
use mmex_lib::domain::payees::PayeeId;
use mmex_lib::domain::transactions::{
    Transaction, TransactionCode, TransactionId, TransactionStatus,
};
use mmex_lib::domain::types::{AccountId, Money};
use rust_decimal_macros::dec;

#[test]
fn test_transaction_full_crud() {
    let ctx = common::setup_test_db();
    let service = ctx.transactions();

    let mut tx = Transaction {
        id: TransactionId { v1: 0 },
        account_id: AccountId { v1: 1 },
        to_account_id: None,
        payee_id: PayeeId { v1: 1 },
        trans_code: TransactionCode::Withdrawal,
        amount: Money(dec!(50.0)),
        status: TransactionStatus::None,
        transaction_number: None,
        notes: None,
        category_id: None,
        date: Some(NaiveDate::from_ymd_opt(2026, 2, 25).unwrap()),
        to_amount: None,
    };

    // 1. Create
    let created = service.create_transaction(&tx).unwrap();
    tx.id = created.id;

    // 2. Update
    tx.amount = Money(dec!(75.0));
    service.update_transaction(&tx).expect("Failed update");
    let found = service.get_transaction_by_id(tx.id).unwrap().unwrap();
    assert_eq!(found.amount.0, dec!(75.0));

    // 3. Delete
    service.delete_transaction(tx.id).expect("Failed delete");
    let after_delete = service.get_transaction_by_id(tx.id).unwrap();
    assert!(after_delete.is_none());
}
