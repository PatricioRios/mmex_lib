use serde::{Deserialize, Serialize};
use crate::error::MmexError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PayeeId(pub i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payee {
    pub id: PayeeId,
    pub name: String,
    pub category_id: Option<i32>, // CATEGID
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
}
