use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Identificador único para una cuenta.
#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId {
    /// Valor bruto del ID en la base de datos legacy.
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

/// Identificador único para una transacción.
#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId {
    /// Valor bruto del ID en la base de datos legacy.
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

/// Identificador único para una etiqueta (Tag).
#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagId {
    /// Valor bruto del ID en la base de datos legacy.
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

/// Identificador único para un beneficiario (Payee).
#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PayeeId {
    /// Valor bruto del ID en la base de datos legacy.
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

/// Identificador único para una categoría.
#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CategoryId {
    /// Valor bruto del ID en la base de datos legacy.
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

/// Identificador único para una moneda (Currency).
#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CurrencyId {
    /// Valor bruto del ID en la base de datos legacy.
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

/// Representa una fecha en el formato compatible con MMEX (YYYY-MM-DD).
#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MmexDate {
    /// Fecha almacenada como cadena de texto ISO 8601.
    pub v1: String, // ISO 8601 YYYY-MM-DD
}

impl From<chrono::NaiveDate> for MmexDate {
    fn from(d: chrono::NaiveDate) -> Self {
        Self { v1: d.to_string() }
    }
}

impl MmexDate {
    /// Convierte la fecha de MMEX a un tipo `NaiveDate` de Rust.
    pub fn to_naive_date(&self) -> chrono::NaiveDate {
        use std::str::FromStr;
        chrono::NaiveDate::from_str(&self.v1)
            .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
    }
}

/// Identificador único para una acción (Stock).
#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StockId {
    /// Valor bruto del ID en la base de datos legacy.
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

/// Identificador único para un activo (Asset).
#[derive(uniffi::Record, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId {
    /// Valor bruto del ID en la base de datos legacy.
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

/// Representa un valor monetario con precisión decimal.
/// Se almacena como String para garantizar la compatibilidad entre plataformas vía FFI.
#[derive(uniffi::Record, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money {
    /// Representación en cadena del valor decimal.
    pub v1: String,
}

impl From<Decimal> for Money {
    fn from(d: Decimal) -> Self {
        Self { v1: d.to_string() }
    }
}

impl Money {
    /// Convierte el valor a `rust_decimal::Decimal` para realizar cálculos.
    pub fn to_decimal(&self) -> Decimal {
        use std::str::FromStr;
        Decimal::from_str(&self.v1).unwrap_or(Decimal::ZERO)
    }

    /// Convierte el valor a `f64` (puede haber pérdida de precisión).
    pub fn to_f64(&self) -> f64 {
        use rust_decimal::prelude::ToPrimitive;
        self.to_decimal().to_f64().unwrap_or(0.0)
    }
}
