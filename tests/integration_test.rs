mod common;
use mmex_lib::domain::accounts::{Account, AccountStatus, AccountType};
use mmex_lib::domain::currencies::CurrencyId;
use mmex_lib::domain::types::{AccountId, Money};
use rust_decimal_macros::dec;

#[test]
fn test_account_full_crud() {
    let ctx = common::setup_test_db();
    let service = ctx.accounts();

    let mut acc = Account {
        id: AccountId { v1: 0 },
        name: "Test Account".to_string(),
        account_type: AccountType::Checking,
        account_num: None,
        status: AccountStatus::Open,
        notes: None,
        initial_balance: Money::from(dec!(100.0)),
        currency_id: CurrencyId { v1: 1 },
        favorite: false,
    };

    // 1. Create
    let created = service.create_account(&acc).unwrap();
    acc.id = created.id;

    // 2. Update
    acc.name = "Updated Account".to_string();
    service.update_account(&acc).expect("Failed update");
    let found = service.get_account_by_id(acc.id).unwrap().unwrap();
    assert_eq!(found.name, "Updated Account");

    // 3. Delete
    service.delete_account(acc.id).expect("Failed delete");
    let after_delete = service.get_account_by_id(acc.id).unwrap();
    assert!(after_delete.is_none());
}
