mod common;

#[test]
fn test_tag_full_crud() {
    let ctx = common::setup_test_db();
    let service = ctx.tags();
    
    // 1. Create
    let mut tag = service.create_tag("Original").unwrap();
    
    // 2. Update
    tag.name = "Updated".to_string();
    service.update_tag(&tag).expect("Failed update");
    let found = service.get_tag_by_id(tag.id).unwrap().unwrap();
    assert_eq!(found.name, "Updated");
    
    // 3. Delete
    service.delete_tag(tag.id).expect("Failed delete");
    let after_delete = service.get_tag_by_id(tag.id).unwrap();
    assert!(after_delete.is_none());
}
