use mmex_lib::api::MmexContext;

#[test]
fn test_tag_crud_integration() {
    let ctx = MmexContext::open_memory().unwrap();
    
    // Setup esquema legacy para Tags
    let schema = "
        CREATE TABLE TAGS_V1 (
            TAGID INTEGER PRIMARY KEY AUTOINCREMENT,
            TAGNAME TEXT NOT NULL UNIQUE
        );
    ";
    ctx.execute_setup(schema).expect("Failed to setup Tags table");
    
    let service = ctx.tags();
    
    // 1. Crear Tag
    let tag = service.create_tag("Facturas").expect("Failed to create tag");
    assert_eq!(tag.name, "Facturas");
    assert!(tag.id.0 > 0);
    
    // 2. Obtener todos los tags
    let tags = service.get_all_tags().expect("Failed to get tags");
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].name, "Facturas");
    
    // 3. Obtener por ID
    let found = service.get_tag_by_id(tag.id).expect("Failed to get by id");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Facturas");
}
