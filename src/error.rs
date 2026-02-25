use thiserror::Error;

#[derive(Error, Debug)]
pub enum MmexError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Encryption error: {0}")]
    Crypto(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Mapping error from legacy schema: {0}")]
    LegacyMapping(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
