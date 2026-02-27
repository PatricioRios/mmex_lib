use serde::{Deserialize, Serialize};
use crate::domain::types::{Money};
use crate::domain::currencies::CurrencyId;
use chrono::NaiveDate;
use crate::error::MmexError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId(pub i64);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AssetStatus {
    Open,
    Closed,
    Unknown(String),
}

impl From<String> for AssetStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Open" => Self::Open,
            "Closed" => Self::Closed,
            _ => Self::Unknown(s),
        }
    }
}

impl ToString for AssetStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Open => "Open".to_string(),
            Self::Closed => "Closed".to_string(),
            Self::Unknown(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: AssetId,
    pub name: String,
    pub start_date: NaiveDate,
    pub status: AssetStatus,
    pub currency_id: Option<CurrencyId>,
    pub value_change_mode: Option<String>, // Percentage, Linear
    pub value: Money,
    pub value_change: Option<String>, // None, Appreciates, Depreciates
    pub notes: Option<String>,
    pub value_change_rate: f64,
    pub asset_type: Option<String>, // Property, Automobile, etc.
}

pub trait AssetRepository {
    fn find_all(&self) -> Result<Vec<Asset>, MmexError>;
    fn find_by_id(&self, id: AssetId) -> Result<Option<Asset>, MmexError>;
    fn insert(&self, asset: &Asset) -> Result<Asset, MmexError>;
    fn update(&self, asset: &Asset) -> Result<(), MmexError>;
    fn delete(&self, id: AssetId) -> Result<(), MmexError>;
}
