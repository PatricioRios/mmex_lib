mod common;
use mmex_lib::domain::types::TransactionId;

#[test]
fn test_transaction_tag_linking() {
    let ctx = common::setup_test_db();
    let tx_service = ctx.transactions();
    let tag_service = ctx.tags();

    // Necesitamos una transacción existente para vincular
    // tables.sql no inserta transacciones por defecto, así que creamos una mínima
    let tx_id = TransactionId { v1: 100 };
    ctx.execute_setup("INSERT INTO CHECKINGACCOUNT_V1 (TRANSID, ACCOUNTID, PAYEEID, TRANSCODE, TRANSAMOUNT) VALUES (100, 1, 1, 'Withdrawal', 50.0);").unwrap();

    // 1. Crear Tag
    let tag = tag_service
        .create_tag("Viajes")
        .expect("Failed to create tag");

    // 2. Vincular
    tx_service.link_tag(tx_id, tag.id).expect("Failed to link");

    // 3. Recuperar y verificar
    let tags = tx_service
        .get_tags_for_transaction(tx_id)
        .expect("Failed to get linked tags");
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].name, "Viajes");

    // 4. Desvincular y verificar
    tx_service
        .unlink_tag(tx_id, tag.id)
        .expect("Failed to unlink");
    let tags_after = tx_service
        .get_tags_for_transaction(tx_id)
        .expect("Failed to get tags after");
    assert_eq!(tags_after.len(), 0);
}
