use serde::{Deserialize, Serialize};
use crate::error::MmexError;

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
    fn get_metadata(&self, name: &str) -> Result<Option<String>, MmexError>;
    fn get_setting(&self, name: &str) -> Result<Option<String>, MmexError>;
    fn set_setting(&self, name: &str, value: &str) -> Result<(), MmexError>;
}
