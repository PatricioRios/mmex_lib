mod common;
use mmex_lib::domain::currencies::{Currency, CurrencyId};
use rust_decimal_macros::dec;
use mmex_lib::domain::Money;

#[test]
fn test_currency_full_crud() {
    let ctx = common::setup_test_db();
    let service = ctx.currencies();

    let mut curr = Currency {
        id: CurrencyId { v1: 0 },
        name: "Test Coin".to_string(),
        pfx_symbol: None,
        sfx_symbol: None,
        decimal_point: Some(".".to_string()),
        group_separator: Some(",".to_string()),
        unit_name: None,
        cent_name: None,
        scale: 100,
        base_conv_rate: Money::from(dec!(1.0)),
        symbol: "TST".to_string(),
        currency_type: "Fiat".to_string(),
    };

    // 1. Create
    let created = service.create_currency(&curr).unwrap();
    curr.id = created.id;

    // 2. Update
    curr.name = "Updated Coin".to_string();
    service.update_currency(&curr).expect("Failed update");
    let found = service.get_currency_by_id(curr.id).unwrap().unwrap();
    assert_eq!(found.name, "Updated Coin");

    // 3. Delete
    service.delete_currency(curr.id).expect("Failed delete");
    let after_delete = service.get_currency_by_id(curr.id).unwrap();
    assert!(after_delete.is_none());
}
