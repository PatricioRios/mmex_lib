use serde::{Deserialize, Serialize};
use crate::error::MmexError;
pub use crate::domain::types::PayeeId;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn find_all(&self) -> Result<Vec<Payee>, MmexError>;
    fn find_by_id(&self, id: PayeeId) -> Result<Option<Payee>, MmexError>;
    fn insert(&self, payee: &Payee) -> Result<Payee, MmexError>;
    fn update(&self, payee: &Payee) -> Result<(), MmexError>;
    fn delete(&self, id: PayeeId) -> Result<(), MmexError>;
}
