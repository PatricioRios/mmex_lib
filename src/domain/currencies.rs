use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use crate::error::MmexError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CurrencyId(pub i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub base_conv_rate: Decimal,
    pub symbol: String, // CURRENCY_SYMBOL (e.g., USD, EUR)
    pub currency_type: String, // Fiat, Crypto
}

pub trait CurrencyRepository {
    fn find_all(&self) -> Result<Vec<Currency>, MmexError>;
    fn find_by_id(&self, id: CurrencyId) -> Result<Option<Currency>, MmexError>;
    fn find_by_symbol(&self, symbol: &str) -> Result<Option<Currency>, MmexError>;
}
