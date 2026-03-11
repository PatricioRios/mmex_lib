uniffi::setup_scaffolding!();
pub mod api;
pub mod domain;
pub mod ffi;
pub mod infrastructure;
pub mod services;

pub use api::MmexContext;
pub use domain::error::MmexError;
pub use ffi::engine::MmexEngine;
