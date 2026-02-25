use mmex_lib::api::MmexContext;
use rust_decimal_macros::dec;

#[test]
fn test_get_all_accounts_integration() {
    // 1. Setup DB en memoria con esquema MMEX legacy
    let ctx = MmexContext::open_memory().unwrap();
    
    let schema = "
        CREATE TABLE ACCOUNTLIST_V1 (
            ACCOUNTID INTEGER PRIMARY KEY,
            ACCOUNTNAME TEXT NOT NULL,
            INITIALBAL TEXT,
            ACCOUNTTYPE TEXT,
            ACCOUNTSTATUS TEXT
        );
        INSERT INTO ACCOUNTLIST_V1 (ACCOUNTID, ACCOUNTNAME, INITIALBAL, ACCOUNTTYPE, ACCOUNTSTATUS)
        VALUES (1, 'Test Account', '100.50', 'Checking', 'Open');
    ";
    
    ctx.execute_setup(schema).expect("Failed to setup test database");
    
    // 2. Ejecutar lógica a través del servicio
    let service = ctx.accounts();
    let accounts = service.get_all_accounts().expect("Failed to get accounts");
    
    // 3. Validar resultados
    assert_eq!(accounts.len(), 1);
    let acc = &accounts[0];
    assert_eq!(acc.name, "Test Account");
    assert_eq!(acc.initial_balance.0, dec!(100.50));
    assert_eq!(acc.account_type, "Checking");
    assert_eq!(acc.status, "Open");
}
