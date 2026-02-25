use mmex_lib::api::MmexContext;
use mmex_lib::domain::categories::CategoryId;

#[test]
fn test_category_hierarchy_integration() {
    let ctx = MmexContext::open_memory().unwrap();
    
    // Setup desde tables.sql
    let schema = "
        CREATE TABLE CATEGORY_V1 (
            CATEGID INTEGER PRIMARY KEY AUTOINCREMENT,
            CATEGNAME TEXT NOT NULL COLLATE NOCASE,
            ACTIVE INTEGER,
            PARENTID INTEGER
        );
        -- Insertar Padre (Bills)
        INSERT INTO CATEGORY_V1 (CATEGID, CATEGNAME, ACTIVE, PARENTID) VALUES (1, 'Bills', 1, -1);
        -- Insertar Hijo (Telephone)
        INSERT INTO CATEGORY_V1 (CATEGID, CATEGNAME, ACTIVE, PARENTID) VALUES (2, 'Telephone', 1, 1);
    ";
    ctx.execute_setup(schema).expect("Failed to setup Categories table");
    
    let service = ctx.categories();
    
    // 1. Obtener todas
    let all = service.get_all_categories().expect("Failed to list");
    assert_eq!(all.len(), 2);
    
    // 2. Validar Padre
    let bills = service.get_category_by_id(CategoryId(1)).expect("Error getting parent").unwrap();
    assert_eq!(bills.name, "Bills");
    assert!(bills.parent_id.is_none());
    
    // 3. Validar Hijo
    let sub = service.get_subcategories(CategoryId(1)).expect("Error getting subs");
    assert_eq!(sub.len(), 1);
    assert_eq!(sub[0].name, "Telephone");
    assert_eq!(sub[0].parent_id, Some(CategoryId(1)));
}
