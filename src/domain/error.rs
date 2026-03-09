use thiserror::Error;

#[derive(Error, Debug)]
pub enum MmexError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Encryption error (SQLCipher): {0}")]
    Crypto(String),

    #[error("Mapping error from legacy schema: {0}")]
    LegacyMapping(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

// Implementación manual de From<rusqlite::Error>
impl From<rusqlite::Error> for MmexError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Database(e.to_string())
    }
}
