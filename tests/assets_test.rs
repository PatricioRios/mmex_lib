mod common;
use chrono::NaiveDate;
use mmex_lib::domain::assets::{Asset, AssetId, AssetStatus};
use mmex_lib::domain::types::Money;
use rust_decimal_macros::dec;
use mmex_lib::domain::MmexDate;

#[test]
fn test_asset_full_crud() {
    let ctx = common::setup_test_db();
    let service = ctx.assets();

    let mut asset = Asset {
        id: AssetId { v1: 0 },
        name: "Casa de Playa".to_string(),
        start_date: MmexDate::from(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
        status: AssetStatus::Open,
        currency_id: Some(mmex_lib::domain::currencies::CurrencyId { v1: 1 }),
        value_change_mode: Some("Linear".into()),
        value: Money::from(dec!(250000.0)),
        value_change: Some("Appreciates".into()),
        notes: Some("Inversión inmobiliaria".into()),
        value_change_rate: 5.0,
        asset_type: Some("Property".into()),
    };

    // 1. Create
    let created = service.create_asset(&asset).expect("Failed create");
    asset.id = created.id;
    assert!(asset.id.v1 > 0);

    // 2. Update
    asset.name = "Casa de Playa Actualizada".to_string();
    asset.value = Money::from(dec!(260000.0));
    service.update_asset(&asset).expect("Failed update");
    let found = service.get_asset_by_id(asset.id).unwrap().unwrap();
    assert_eq!(found.name, "Casa de Playa Actualizada");
    assert_eq!(found.value.to_decimal(), dec!(260000.0));

    // 3. Delete
    service.delete_asset(asset.id).expect("Failed delete");
    let after_delete = service.get_asset_by_id(asset.id).unwrap();
    assert!(after_delete.is_none());
}
