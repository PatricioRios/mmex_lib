pub use crate::domain::types::{CurrencyId, Money};
use crate::MmexError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(uniffi::Error, Error, Debug)]
pub enum CurrencyError {
    #[error("Currency common error: {0}")]
    Common(#[from] MmexError),

    #[error("Currency not found: {0}")]
    NotFound(CurrencyId),

    #[error("Currency name is required")]
    NameRequired,
}

#[derive(uniffi::Record, Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub id: CurrencyId,
    pub name: String,
    pub pfx_symbol: Option<String>,
    pub sfx_symbol: Option<String>,
    pub decimal_point: Option<String>,
    pub group_separator: Option<String>,
    pub unit_name: Option<String>,
    pub cent_name: Option<String>,
    pub scale: i32,
    pub base_conv_rate: Money,
    pub symbol: String,        // CURRENCY_SYMBOL (e.g., USD, EUR)
    pub currency_type: String, // Fiat, Crypto
}

pub trait CurrencyRepository {
    fn find_all(&self) -> Result<Vec<Currency>, CurrencyError>;
    fn find_by_id(&self, id: CurrencyId) -> Result<Option<Currency>, CurrencyError>;
    fn find_by_symbol(&self, symbol: &str) -> Result<Option<Currency>, CurrencyError>;
    fn insert(&self, currency: &Currency) -> Result<Currency, CurrencyError>;
    fn update(&self, currency: &Currency) -> Result<(), CurrencyError>;
    fn delete(&self, id: CurrencyId) -> Result<(), CurrencyError>;
}

impl From<CurrencyError> for MmexError {
    fn from(e: CurrencyError) -> Self {
        match e {
            CurrencyError::Common(c) => c,
            _ => MmexError::Internal(e.to_string()),
        }
    }
}
