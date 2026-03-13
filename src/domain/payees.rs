pub use crate::domain::types::PayeeId;
use crate::MmexError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PayeeError {
    #[error("Payee common error: {0}")]
    Common(#[from] MmexError),

    #[error("Payee not found: {0}")]
    NotFound(PayeeId),

    #[error("Payee name is required")]
    NameRequired,
}

#[derive(uniffi::Record, Debug, Clone, Serialize, Deserialize)]
pub struct Payee {
    pub id: PayeeId,
    pub name: String,
    pub category_id: Option<i64>, // CATEGID
    pub number: Option<String>,
    pub website: Option<String>,
    pub notes: Option<String>,
    pub active: bool,
    pub pattern: Option<String>,
}

pub trait PayeeRepository {
    fn find_all(&self) -> Result<Vec<Payee>, PayeeError>;
    fn find_by_id(&self, id: PayeeId) -> Result<Option<Payee>, PayeeError>;
    fn insert(&self, payee: &Payee) -> Result<Payee, PayeeError>;
    fn update(&self, payee: &Payee) -> Result<(), PayeeError>;
    fn delete(&self, id: PayeeId) -> Result<(), PayeeError>;
}

impl From<PayeeError> for MmexError {
    fn from(e: PayeeError) -> Self {
        match e {
            PayeeError::Common(c) => c,
            _ => MmexError::Internal(e.to_string()),
        }
    }
}
