use serde::{Deserialize, Serialize};
use crate::domain::types::{Money};
use chrono::NaiveDate;
use crate::error::MmexError;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StockId(pub i64);

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
    fn find_all(&self) -> Result<Vec<Stock>, MmexError>;
    fn find_by_id(&self, id: StockId) -> Result<Option<Stock>, MmexError>;
    fn insert(&self, stock: &Stock) -> Result<Stock, MmexError>;
    fn update(&self, stock: &Stock) -> Result<(), MmexError>;
    fn delete(&self, id: StockId) -> Result<(), MmexError>;
}
