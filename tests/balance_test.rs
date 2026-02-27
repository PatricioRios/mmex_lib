mod common;
use mmex_lib::domain::accounts::{Account, AccountType, AccountStatus};
use mmex_lib::domain::transactions::{Transaction, TransactionCode, TransactionStatus};
use mmex_lib::domain::types::{AccountId, Money, TransactionId};
use mmex_lib::domain::payees::PayeeId;
use rust_decimal_macros::dec;

#[test]
fn test_account_balance_calculation() {
    let ctx = common::setup_test_db();
    let account_service = ctx.accounts();
    let tx_service = ctx.transactions();
    
    // 1. Crear Cuenta Principal (Balance Inicial: 1000)
    let acc = Account {
        id: AccountId(1), name: "Bank".into(), account_type: AccountType::Checking, account_num: None, status: AccountStatus::Open, notes: None, initial_balance: Money(dec!(1000.0)), currency_id: mmex_lib::domain::currencies::CurrencyId(1), favorite: false,
    };
    account_service.create_account(&acc).unwrap();
    
    // 2. Crear Cuenta Secundaria para transferencias
    let acc2 = Account {
        id: AccountId(2), name: "Savings".into(), account_type: AccountType::Checking, account_num: None, status: AccountStatus::Open, notes: None, initial_balance: Money(dec!(0.0)), currency_id: mmex_lib::domain::currencies::CurrencyId(1), favorite: false,
    };
    account_service.create_account(&acc2).unwrap();

    // 3. Realizar Movimientos
    
    // Depósito: +500
    tx_service.create_transaction(&Transaction {
        id: TransactionId(0), account_id: AccountId(1), to_account_id: None, payee_id: PayeeId(1), trans_code: TransactionCode::Deposit, amount: Money(dec!(500.0)), status: TransactionStatus::None, transaction_number: None, notes: None, category_id: None, date: None, to_amount: None,
    }).unwrap();

    // Retiro: -200
    tx_service.create_transaction(&Transaction {
        id: TransactionId(0), account_id: AccountId(1), to_account_id: None, payee_id: PayeeId(1), trans_code: TransactionCode::Withdrawal, amount: Money(dec!(200.0)), status: TransactionStatus::None, transaction_number: None, notes: None, category_id: None, date: None, to_amount: None,
    }).unwrap();

    // Transferencia Entrante: +300 (Desde Cuenta 2 a Cuenta 1)
    tx_service.create_transaction(&Transaction {
        id: TransactionId(0), account_id: AccountId(2), to_account_id: Some(AccountId(1)), payee_id: PayeeId(1), trans_code: TransactionCode::Transfer, amount: Money(dec!(300.0)), status: TransactionStatus::None, transaction_number: None, notes: None, category_id: None, date: None, to_amount: None,
    }).unwrap();

    // Transferencia Saliente: -100 (Desde Cuenta 1 a Cuenta 2)
    tx_service.create_transaction(&Transaction {
        id: TransactionId(0), account_id: AccountId(1), to_account_id: Some(AccountId(2)), payee_id: PayeeId(1), trans_code: TransactionCode::Transfer, amount: Money(dec!(100.0)), status: TransactionStatus::None, transaction_number: None, notes: None, category_id: None, date: None, to_amount: None,
    }).unwrap();

    // 4. Calcular y Validar
    let balance = account_service.get_account_balance(AccountId(1)).expect("Failed to calculate balance");
    
    // Balance esperado: 1000 (init) + 500 (dep) - 200 (with) + 300 (trans in) - 100 (trans out) = 1500
    assert_eq!(balance.current_balance.0, dec!(1500.0));
    assert_eq!(balance.total_deposits.0, dec!(800.0)); // 500 + 300
    assert_eq!(balance.total_withdrawals.0, dec!(300.0)); // 200 + 100
}
