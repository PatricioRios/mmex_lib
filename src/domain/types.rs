use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId {
    pub v1: i64,
}

impl AccountId {
    pub fn new(v1: i64) -> Self {
        Self { v1 }
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v1)
    }
}

#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId {
    pub v1: i64,
}

impl TransactionId {
    pub fn new(v1: i64) -> Self {
        Self { v1 }
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v1)
    }
}

#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagId {
    pub v1: i64,
}

impl TagId {
    pub fn new(v1: i64) -> Self {
        Self { v1 }
    }
}

impl fmt::Display for TagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v1)
    }
}

#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PayeeId {
    pub v1: i64,
}

impl PayeeId {
    pub fn new(v1: i64) -> Self {
        Self { v1 }
    }
}

impl fmt::Display for PayeeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v1)
    }
}

#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CategoryId {
    pub v1: i64,
}

impl CategoryId {
    pub fn new(v1: i64) -> Self {
        Self { v1 }
    }
}

impl fmt::Display for CategoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v1)
    }
}

#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CurrencyId {
    pub v1: i64,
}

impl CurrencyId {
    pub fn new(v1: i64) -> Self {
        Self { v1 }
    }
}

impl fmt::Display for CurrencyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v1)
    }
}

#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MmexDate {
    pub v1: String, // ISO 8601 YYYY-MM-DD
}

impl From<chrono::NaiveDate> for MmexDate {
    fn from(d: chrono::NaiveDate) -> Self {
        Self { v1: d.to_string() }
    }
}

impl MmexDate {
    pub fn to_naive_date(&self) -> chrono::NaiveDate {
        use std::str::FromStr;
        chrono::NaiveDate::from_str(&self.v1)
            .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
    }
}

#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StockId {
    pub v1: i64,
}

impl StockId {
    pub fn new(v1: i64) -> Self {
        Self { v1 }
    }
}

impl fmt::Display for StockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v1)
    }
}

#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId {
    pub v1: i64,
}

impl AssetId {
    pub fn new(v1: i64) -> Self {
        Self { v1 }
    }
}

impl fmt::Display for AssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v1)
    }
}

#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]

pub struct Money {
    pub v1: String,
}

impl From<Decimal> for Money {
    fn from(d: Decimal) -> Self {
        Self { v1: d.to_string() }
    }
}

impl Money {
    pub fn to_decimal(&self) -> Decimal {
        use std::str::FromStr;
        Decimal::from_str(&self.v1).unwrap_or(Decimal::ZERO)
    }

    pub fn to_f64(&self) -> f64 {
        use rust_decimal::prelude::ToPrimitive;
        self.to_decimal().to_f64().unwrap_or(0.0)
    }
}
