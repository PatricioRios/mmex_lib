use mmex_lib::api::MmexContext;

#[test]
fn test_payee_integration() {
    let ctx = MmexContext::open_memory().unwrap();
    
    // Setup exacto desde tables.sql
    let schema = "
        CREATE TABLE PAYEE_V1(
            PAYEEID integer primary key AUTOINCREMENT
            , PAYEENAME TEXT COLLATE NOCASE NOT NULL UNIQUE
            , CATEGID integer
            , NUMBER TEXT
            , WEBSITE TEXT
            , NOTES TEXT
            , ACTIVE integer
            , PATTERN TEXT DEFAULT ''
        );
    ";
    ctx.execute_setup(schema).expect("Failed to setup Payees table");
    
    let service = ctx.payees();
    
    // 1. Crear Payee
    let payee = service.create_payee("Supermercado Central").expect("Failed to create payee");
    assert_eq!(payee.name, "Supermercado Central");
    assert!(payee.active);
    
    // 2. Listar
    let all = service.get_all_payees().expect("Failed to list");
    assert_eq!(all.len(), 1);
    
    // 3. Buscar por ID
    let found = service.get_payee_by_id(payee.id).expect("Failed to find");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Supermercado Central");
}
