use crate::MmexError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SupportError {
    #[error("Support common error: {0}")]
    Common(#[from] MmexError),

    #[error("Metadata or setting not found: {0}")]
    NotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbMetadata {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSetting {
    pub name: String,
    pub value: Option<String>,
}

pub trait SupportRepository {
    fn get_metadata(&self, name: &str) -> Result<Option<String>, SupportError>;
    fn get_setting(&self, name: &str) -> Result<Option<String>, SupportError>;
    fn set_setting(&self, name: &str, value: &str) -> Result<(), SupportError>;
}

impl From<SupportError> for MmexError {
    fn from(e: SupportError) -> Self {
        match e {
            SupportError::Common(c) => c,
            _ => MmexError::Internal(e.to_string()),
        }
    }
}
