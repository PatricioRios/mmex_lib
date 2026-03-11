use mmex_lib::api::MmexContext;
use std::fs;

#[allow(dead_code)]
pub fn setup_test_db() -> MmexContext {
    let ctx = MmexContext::open_memory().expect("Failed to open memory DB");
    let schema = fs::read_to_string("tables.sql").expect("Could not read tables.sql");
    ctx.execute_setup(&schema)
        .expect("Failed to execute tables.sql");
    ctx
}
