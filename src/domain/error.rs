use thiserror::Error;

#[derive(uniffi::Error, Error, Debug)]
pub enum MmexError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Database is full: {0}")]
    DiskFull(String),

    #[error("Database is locked/busy: {0}")]
    DatabaseBusy(String),

    #[error("Database is corrupt: {0}")]
    DatabaseCorrupt(String),

    #[error("Unique constraint violation: {0}")]
    UniqueConstraint(String),

    #[error("Foreign key constraint violation: {0}")]
    ForeignKeyConstraint(String),

    #[error("Encryption error (SQLCipher): {0}")]
    Crypto(String),

    #[error("Mapping error from legacy schema: {0}")]
    LegacyMapping(String),

    #[error("Resource not found")]
    NotFound,

    #[error("Internal error: {0}")]
    Internal(String),
}

// Implementación mejorada de From<rusqlite::Error>
impl From<rusqlite::Error> for MmexError {
    fn from(e: rusqlite::Error) -> Self {
        match e {
            rusqlite::Error::SqliteFailure(err, msg) => {
                let message = msg.clone().unwrap_or_else(|| err.to_string());
                match err.code {
                    rusqlite::ErrorCode::DiskFull => Self::DiskFull(message),
                    rusqlite::ErrorCode::DatabaseBusy => Self::DatabaseBusy(message),
                    rusqlite::ErrorCode::DatabaseCorrupt => Self::DatabaseCorrupt(message),
                    rusqlite::ErrorCode::ConstraintViolation => {
                        if message.contains("UNIQUE") {
                            Self::UniqueConstraint(message)
                        } else if message.contains("FOREIGN KEY") {
                            Self::ForeignKeyConstraint(message)
                        } else {
                            Self::Database(message)
                        }
                    }
                    _ => Self::Database(message),
                }
            }
            rusqlite::Error::QueryReturnedNoRows => Self::NotFound,
            _ => Self::Database(e.to_string()),
        }
    }
}
