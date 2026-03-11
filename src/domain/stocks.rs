use crate::domain::types::Money;
use crate::MmexError;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StockError {
    #[error("Stock common error: {0}")]
    Common(#[from] MmexError),

    #[error("Stock not found: {0}")]
    NotFound(StockId),
}

#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StockId {
    pub v1: i64,
}

impl std::fmt::Display for StockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.v1)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub id: StockId,
    pub held_at: i64, // Referencia a cuenta
    pub purchase_date: NaiveDate,
    pub name: String,
    pub symbol: Option<String>,
    pub num_shares: Decimal,
    pub purchase_price: Money,
    pub notes: Option<String>,
    pub current_price: Money,
    pub value: Money,
    pub commission: Money,
}

pub trait StockRepository {
    fn find_all(&self) -> Result<Vec<Stock>, StockError>;
    fn find_by_id(&self, id: StockId) -> Result<Option<Stock>, StockError>;
    fn insert(&self, stock: &Stock) -> Result<Stock, StockError>;
    fn update(&self, stock: &Stock) -> Result<(), StockError>;
    fn delete(&self, id: StockId) -> Result<(), StockError>;
}

impl From<StockError> for MmexError {
    fn from(e: StockError) -> Self {
        match e {
            StockError::Common(c) => c,
            _ => MmexError::Internal(e.to_string()),
        }
    }
}
