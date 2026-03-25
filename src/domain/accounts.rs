pub use crate::domain::types::{AccountId, CurrencyId, Money};
use crate::MmexError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errores específicos relacionados con la gestión de cuentas.
#[derive(uniffi::Error, Error, Debug)]
pub enum AccountError {
    /// Error común propagado desde el core.
    #[error("Account common error: {0}")]
    Common(#[from] MmexError),

    /// La cuenta con el ID proporcionado no existe.
    #[error("Account not found: {0}")]
    NotFound(AccountId),

    /// Intento de crear una cuenta sin nombre.
    #[error("Account name is required")]
    NameRequired,
}

/// Tipos de cuentas financieras soportadas por MMEX.
#[derive(uniffi::Enum, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountType {
    /// Efectivo.
    Cash,
    /// Cuenta corriente o bancaria estándar.
    Checking,
    /// Cuenta a plazo o ahorros.
    Term,
    /// Cuenta de inversión.
    Investment,
    /// Tarjeta de crédito.
    CreditCard,
    /// Préstamo.
    Loan,
    /// Activo fijo.
    Asset,
    /// Acciones y valores.
    Shares,
    /// Tipo de cuenta no reconocido o personalizado.
    Unknown(String),
}

impl From<String> for AccountType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Cash" => Self::Cash,
            "Checking" => Self::Checking,
            "Term" => Self::Term,
            "Investment" => Self::Investment,
            "Credit Card" => Self::CreditCard,
            "Loan" => Self::Loan,
            "Asset" => Self::Asset,
            "Shares" => Self::Shares,
            _ => Self::Unknown(s),
        }
    }
}

impl ToString for AccountType {
    fn to_string(&self) -> String {
        match self {
            Self::Cash => "Cash".to_string(),
            Self::Checking => "Checking".to_string(),
            Self::Term => "Term".to_string(),
            Self::Investment => "Investment".to_string(),
            Self::CreditCard => "Credit Card".to_string(),
            Self::Loan => "Loan".to_string(),
            Self::Asset => "Asset".to_string(),
            Self::Shares => "Shares".to_string(),
            Self::Unknown(s) => s.clone(),
        }
    }
}

/// Estados posibles de una cuenta.
#[derive(uniffi::Enum, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountStatus {
    /// Cuenta activa y en uso.
    Open,
    /// Cuenta cerrada o inactiva.
    Closed,
    /// Estado no reconocido.
    Unknown(String),
}

impl From<String> for AccountStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Open" => Self::Open,
            "Closed" => Self::Closed,
            _ => Self::Unknown(s),
        }
    }
}

impl ToString for AccountStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Open => "Open".to_string(),
            Self::Closed => "Closed".to_string(),
            Self::Unknown(s) => s.clone(),
        }
    }
}

/// Representa una cuenta financiera completa en el sistema.
#[derive(uniffi::Record, Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Identificador único de la cuenta.
    pub id: AccountId,
    /// Nombre descriptivo que identifica la cuenta.
    pub name: String,
    /// Clasificación de la cuenta (Checking, Cash, etc.).
    pub account_type: AccountType,
    /// Número de cuenta bancaria u otro identificador externo (opcional).
    pub account_num: Option<String>,
    /// Estado operativo de la cuenta.
    pub status: AccountStatus,
    /// Comentarios o descripciones adicionales.
    pub notes: Option<String>,
    /// Saldo con el que se dio de alta la cuenta.
    pub initial_balance: Money,
    /// Referencia a la moneda principal de la cuenta.
    pub currency_id: CurrencyId,
    /// Indica si el usuario la ha marcado como favorita.
    pub favorite: bool,
}

/// Estructura para realizar actualizaciones parciales en una cuenta.
/// Cada campo opcional representa un valor que puede o no ser modificado.
#[derive(uniffi::Record, Debug, Clone, Default)]
pub struct AccountUpdate {
    pub name: Option<String>,
    pub account_type: Option<AccountType>,
    pub account_num: Option<String>,
    pub status: Option<AccountStatus>,
    pub notes: Option<String>,
    pub initial_balance: Option<Money>,
    pub currency_id: Option<CurrencyId>,
    pub favorite: Option<bool>,
}

/// Interfaz para la persistencia de datos de cuentas.
pub trait AccountRepository {
    /// Recupera todas las cuentas registradas.
    fn find_all(&self) -> Result<Vec<Account>, AccountError>;
    /// Busca una cuenta específica por su identificador.
    fn find_by_id(&self, id: AccountId) -> Result<Option<Account>, AccountError>;
    /// Inserta una nueva cuenta en el sistema.
    fn insert(&self, account: &Account) -> Result<Account, AccountError>;
    /// Actualiza todos los campos de una cuenta existente.
    fn update(&self, account: &Account) -> Result<(), AccountError>;
    /// Realiza una actualización selectiva de los campos de una cuenta.
    fn update_partial(&self, id: AccountId, update: AccountUpdate) -> Result<(), AccountError>;
    /// Elimina una cuenta del sistema.
    fn delete(&self, id: AccountId) -> Result<(), AccountError>;
}

/// Resume el estado financiero de una cuenta calculando sus flujos.
#[derive(uniffi::Record, Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    /// Referencia al ID de la cuenta analizada.
    pub account_id: AccountId,
    /// Saldo inicial configurado.
    pub initial_balance: Money,
    /// Suma de todas las transacciones de tipo ingreso.
    pub total_deposits: Money,
    /// Suma de todas las transacciones de tipo gasto o retiro.
    pub total_withdrawals: Money,
    /// Saldo actual calculado (Inicial + Depósitos - Retiros).
    pub current_balance: Money,
}

impl From<AccountError> for MmexError {
    fn from(e: AccountError) -> Self {
        match e {
            AccountError::Common(c) => c,
            _ => MmexError::Internal(e.to_string()),
        }
    }
}
