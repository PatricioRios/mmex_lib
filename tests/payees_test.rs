mod common;

#[test]
fn test_payee_full_crud() {
    let ctx = common::setup_test_db();
    let service = ctx.payees();
    
    // 1. Create
    let mut payee = service.create_payee("Test Payee").unwrap();
    
    // 2. Update
    payee.name = "Updated Payee".to_string();
    service.update_payee(&payee).expect("Failed update");
    let found = service.get_payee_by_id(payee.id).unwrap().unwrap();
    assert_eq!(found.name, "Updated Payee");
    
    // 3. Delete
    service.delete_payee(payee.id).expect("Failed delete");
    let after_delete = service.get_payee_by_id(payee.id).unwrap();
    assert!(after_delete.is_none());
}
