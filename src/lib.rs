pub mod domain;
pub mod infrastructure;
pub mod services;
pub mod api;
pub mod error;

pub use error::MmexError;
pub use api::MmexContext;

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
