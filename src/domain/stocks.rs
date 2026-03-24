pub use crate::domain::types::{MmexDate, Money, StockId};
use crate::MmexError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(uniffi::Error, Error, Debug)]
pub enum StockError {
    #[error("Stock common error: {0}")]
    Common(#[from] MmexError),

    #[error("Stock not found: {0}")]
    NotFound(StockId),
}

#[derive(uniffi::Record, Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub id: StockId,
    pub held_at: i64, // Referencia a cuenta
    pub purchase_date: MmexDate,
    pub name: String,
    pub symbol: Option<String>,
    pub num_shares: Money,
    pub purchase_price: Money,
    pub notes: Option<String>,
    pub current_price: Money,
    pub value: Money,
    pub commission: Money,
}

#[derive(uniffi::Record, Debug, Clone, Default)]
pub struct StockUpdate {
    pub held_at: Option<i64>,
    pub purchase_date: Option<MmexDate>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub num_shares: Option<Money>,
    pub purchase_price: Option<Money>,
    pub notes: Option<String>,
    pub current_price: Option<Money>,
    pub value: Option<Money>,
    pub commission: Option<Money>,
}

pub trait StockRepository {
    fn find_all(&self) -> Result<Vec<Stock>, StockError>;
    fn find_by_id(&self, id: StockId) -> Result<Option<Stock>, StockError>;
    fn insert(&self, stock: &Stock) -> Result<Stock, StockError>;
    fn update(&self, stock: &Stock) -> Result<(), StockError>;
    fn update_partial(&self, id: StockId, update: StockUpdate) -> Result<(), StockError>;
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
