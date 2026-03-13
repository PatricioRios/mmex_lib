mod common;
use chrono::NaiveDate;
use mmex_lib::domain::stocks::{Stock, StockId};
use mmex_lib::domain::types::Money;
use rust_decimal_macros::dec;
use mmex_lib::domain::MmexDate;

#[test]
fn test_stock_full_crud() {
    let ctx = common::setup_test_db();
    let service = ctx.stocks();

    let mut stock = Stock {
        id: StockId { v1: 0 },
        held_at: 1, // Referencia a cuenta
        purchase_date: MmexDate::from(NaiveDate::from_ymd_opt(2023, 5, 10).unwrap()),
        name: "Apple Inc.".to_string(),
        symbol: Some("AAPL".into()),
        num_shares: Money::from(dec!(10.5)),
        purchase_price: Money::from(dec!(150.0)),
        notes: Some("Inversión a largo plazo".into()),
        current_price: Money::from(dec!(180.0)),
        value: Money::from(dec!(1890.0)),
        commission: Money::from(dec!(5.0)),
    };

    // 1. Create
    let created = service.create_stock(&stock).expect("Failed create");
    stock.id = created.id;
    assert!(stock.id.v1 > 0);

    // 2. Update
    stock.name = "Apple Inc. Updated".to_string();
    stock.current_price = Money::from(dec!(190.0));
    service.update_stock(&stock).expect("Failed update");
    let found = service.get_stock_by_id(stock.id).unwrap().unwrap();
    assert_eq!(found.name, "Apple Inc. Updated");
    assert_eq!(found.current_price.to_decimal(), dec!(190.0));

    // 3. Delete
    service.delete_stock(stock.id).expect("Failed delete");
    let after_delete = service.get_stock_by_id(stock.id).unwrap();
    assert!(after_delete.is_none());
}
