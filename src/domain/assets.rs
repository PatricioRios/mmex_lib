use crate::domain::currencies::CurrencyId;
use crate::domain::types::Money;
use crate::MmexError;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssetError {
    #[error("Asset common error: {0}")]
    Common(#[from] MmexError),

    #[error("Asset not found: {0}")]
    NotFound(AssetId),
}

#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId {
    pub v1: i64,
}

impl std::fmt::Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.v1)
    }
}

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
    fn find_all(&self) -> Result<Vec<Asset>, AssetError>;
    fn find_by_id(&self, id: AssetId) -> Result<Option<Asset>, AssetError>;
    fn insert(&self, asset: &Asset) -> Result<Asset, AssetError>;
    fn update(&self, asset: &Asset) -> Result<(), AssetError>;
    fn delete(&self, id: AssetId) -> Result<(), AssetError>;
}

impl From<AssetError> for MmexError {
    fn from(e: AssetError) -> Self {
        match e {
            AssetError::Common(c) => c,
            _ => MmexError::Internal(e.to_string()),
        }
    }
}
