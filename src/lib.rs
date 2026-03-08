pub mod api;
pub mod domain;
pub mod infrastructure;
pub mod services;

pub use api::MmexContext;
pub use domain::error::MmexError;

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
