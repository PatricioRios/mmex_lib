mod common;
use chrono::NaiveDate;
use mmex_lib::domain::payees::PayeeId;
use mmex_lib::domain::scheduled_transactions::ScheduledTransaction;
use mmex_lib::domain::transactions::{TransactionCode, TransactionStatus};
use mmex_lib::domain::types::{AccountId, Money};
use rust_decimal_macros::dec;

#[test]
fn test_scheduled_full_crud() {
    let ctx = common::setup_test_db();
    let service = ctx.scheduled();

    let mut stx = ScheduledTransaction {
        id: 0,
        account_id: AccountId { v1: 1 },
        to_account_id: None,
        payee_id: PayeeId { v1: 1 },
        trans_code: TransactionCode::Withdrawal,
        amount: Money(dec!(120.0)),
        status: TransactionStatus::None,
        transaction_number: None,
        notes: Some("Gimnasio".into()),
        category_id: None,
        trans_date: Some(NaiveDate::from_ymd_opt(2026, 3, 1).unwrap()),
        next_occurrence_date: Some(NaiveDate::from_ymd_opt(2026, 4, 1).unwrap()),
        repeats: 1, // Monthly
        num_occurrences: 12,
        to_trans_amount: None,
    };

    // 1. Create
    let created = service.create_scheduled(&stx).expect("Failed create");
    stx.id = created.id;
    assert!(stx.id > 0);

    // 2. Update
    stx.amount = Money(dec!(130.0));
    service.update_scheduled(&stx).expect("Failed update");
    let found = service.get_scheduled_by_id(stx.id).unwrap().unwrap();
    assert_eq!(found.amount.0, dec!(130.0));

    // 3. Delete
    service.delete_scheduled(stx.id).expect("Failed delete");
    let after_delete = service.get_scheduled_by_id(stx.id).unwrap();
    assert!(after_delete.is_none());
}
