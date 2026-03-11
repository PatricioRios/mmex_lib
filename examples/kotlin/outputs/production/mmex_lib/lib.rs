uniffi::setup_scaffolding!();
pub mod api;
pub mod domain;
pub mod infrastructure;
pub mod services;

pub use api::MmexContext;
pub use domain::error::MmexError;

#[uniffi::export]
pub fn sum(a: i32, b: i32) -> i32 {
    println!("sumando desde Rust: {} + {} = {}", a, b, a + b);
    a + b
}
