mod common;
use mmex_lib::domain::categories::CategoryId;
use mmex_lib::domain::payees::PayeeId;
use mmex_lib::domain::transactions::{
    SplitTransaction, Transaction, TransactionCode, TransactionId, TransactionStatus,
};
use mmex_lib::domain::types::{AccountId, Money};
use rust_decimal_macros::dec;

#[test]
fn test_transaction_splits_integration() {
    let ctx = common::setup_test_db();
    let service = ctx.transactions();

    // 1. Crear Transacción base
    let tx = Transaction {
        id: TransactionId { v1: 0 },
        account_id: AccountId { v1: 1 },
        to_account_id: None,
        payee_id: PayeeId { v1: 1 },
        trans_code: TransactionCode::Withdrawal,
        amount: Money(dec!(100.0)),
        status: TransactionStatus::None,
        transaction_number: None,
        notes: None,
        category_id: None,
        date: None,
        to_amount: None,
    };
    let created_tx = service.create_transaction(&tx).unwrap();

    // 2. Añadir Desgloses (Splits)
    let split1 = SplitTransaction {
        id: 0,
        transaction_id: created_tx.id,
        category_id: Some(CategoryId { v1: 10 }),
        amount: Money(dec!(60.0)),
        notes: Some("Comida".into()),
    };
    let split2 = SplitTransaction {
        id: 0,
        transaction_id: created_tx.id,
        category_id: Some(CategoryId { v1: 11 }),
        amount: Money(dec!(40.0)),
        notes: Some("Bebida".into()),
    };

    service.add_split(&split1).unwrap();
    service.add_split(&split2).unwrap();

    // 3. Verificar recuperación
    let splits = service.get_splits_for_transaction(created_tx.id).unwrap();
    assert_eq!(splits.len(), 2);
    assert_eq!(splits[0].amount.0, dec!(60.0));

    // 4. Borrar transacción y verificar borrado de splits
    service.delete_transaction(created_tx.id).unwrap();
    let after_delete = service.get_splits_for_transaction(created_tx.id).unwrap();
    assert_eq!(after_delete.len(), 0);
}
