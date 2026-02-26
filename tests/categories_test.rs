mod common;

#[test]
fn test_category_full_crud() {
    let ctx = common::setup_test_db();
    let service = ctx.categories();
    
    // 1. Create Root
    let mut root = service.create_category("Root Category", None).unwrap();
    
    // 2. Update
    root.name = "Updated Root".to_string();
    service.update_category(&root).expect("Failed update");
    let found = service.get_category_by_id(root.id).unwrap().unwrap();
    assert_eq!(found.name, "Updated Root");
    
    // 3. Delete
    service.delete_category(root.id).expect("Failed delete");
    let after_delete = service.get_category_by_id(root.id).unwrap();
    assert!(after_delete.is_none());
}
