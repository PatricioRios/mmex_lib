use crate::domain::payees::PayeeId;
pub use crate::domain::types::{AccountId, CategoryId, MmexDate, Money, TransactionId};
use crate::MmexError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errores específicos relacionados con la gestión de transacciones.
#[derive(uniffi::Error, Error, Debug)]
pub enum TransactionError {
    /// Error común propagado desde el core.
    #[error("Transaction common error: {0}")]
    Common(#[from] MmexError),

    /// La transacción con el ID proporcionado no existe.
    #[error("Transaction not found: {0}")]
    NotFound(TransactionId),

    /// El monto de la transacción no es válido (ej: negativo en contextos no permitidos).
    #[error("Invalid transaction amount")]
    InvalidAmount,

    /// Error ocurrido durante la gestión de una transacción dividida (Split).
    #[error("Split error: {0}")]
    SplitError(String),
}

/// Clasificación de la naturaleza de la transacción.
#[derive(uniffi::Enum, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionCode {
    /// Retiro de fondos (Gasto).
    Withdrawal,
    /// Depósito de fondos (Ingreso).
    Deposit,
    /// Transferencia entre cuentas.
    Transfer,
    /// Código de transacción no reconocido.
    Unknown(String),
}

impl From<String> for TransactionCode {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Withdrawal" => Self::Withdrawal,
            "Deposit" => Self::Deposit,
            "Transfer" => Self::Transfer,
            _ => Self::Unknown(s),
        }
    }
}

impl ToString for TransactionCode {
    fn to_string(&self) -> String {
        match self {
            Self::Withdrawal => "Withdrawal".to_string(),
            Self::Deposit => "Deposit".to_string(),
            Self::Transfer => "Transfer".to_string(),
            Self::Unknown(s) => s.clone(),
        }
    }
}

/// Estado de reconciliación o validez de la transacción.
#[derive(uniffi::Enum, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    /// Sin estado específico (Pendiente).
    None,
    /// Reconciliada con el banco.
    Reconciled,
    /// Transacción anulada.
    Void,
    /// Requiere seguimiento.
    FollowUp,
    /// Marcada como duplicada.
    Duplicate,
    /// Estado no reconocido.
    Unknown(String),
}

impl From<String> for TransactionStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "None" | "" => Self::None,
            "Reconciled" => Self::Reconciled,
            "Void" => Self::Void,
            "Follow up" => Self::FollowUp,
            "Duplicate" => Self::Duplicate,
            _ => Self::Unknown(s),
        }
    }
}

impl ToString for TransactionStatus {
    fn to_string(&self) -> String {
        match self {
            Self::None => "".to_string(),
            Self::Reconciled => "R".to_string(),
            Self::Void => "V".to_string(),
            Self::FollowUp => "F".to_string(),
            Self::Duplicate => "D".to_string(),
            Self::Unknown(s) => s.clone(),
        }
    }
}

/// Representa una transacción financiera individual.
#[derive(uniffi::Record, Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Identificador único.
    pub id: TransactionId,
    /// Cuenta de origen (o destino en depósitos).
    pub account_id: AccountId,
    /// Cuenta de destino (solo para transferencias).
    pub to_account_id: Option<AccountId>,
    /// Beneficiario o pagador asociado.
    pub payee_id: PayeeId,
    /// Naturaleza de la operación (Withdrawal, Deposit, Transfer).
    pub trans_code: TransactionCode,
    /// Monto de la transacción en la moneda de la cuenta de origen.
    pub amount: Money,
    /// Estado de la transacción (Reconciliada, Nula, etc.).
    pub status: TransactionStatus,
    /// Número de cheque o referencia externa.
    pub transaction_number: Option<String>,
    /// Comentarios adicionales.
    pub notes: Option<String>,
    /// Clasificación por categoría.
    pub category_id: Option<CategoryId>,
    /// Fecha en la que se realizó la operación.
    pub date: Option<MmexDate>,
    /// Monto en la moneda de destino (solo para transferencias multi-moneda).
    pub to_amount: Option<Money>,
}

/// Estructura para actualizaciones parciales de transacciones.
#[derive(uniffi::Record, Debug, Clone, Default)]
pub struct TransactionUpdate {
    pub account_id: Option<AccountId>,
    pub to_account_id: Option<AccountId>,
    pub payee_id: Option<PayeeId>,
    pub trans_code: Option<TransactionCode>,
    pub amount: Option<Money>,
    pub status: Option<TransactionStatus>,
    pub transaction_number: Option<String>,
    pub notes: Option<String>,
    pub category_id: Option<CategoryId>,
    pub date: Option<MmexDate>,
    pub to_amount: Option<Money>,
}

/// Interfaz para la persistencia de transacciones.
pub trait TransactionRepository {
    /// Obtiene todas las transacciones ordenadas.
    fn find_all(&self) -> Result<Vec<Transaction>, TransactionError>;
    /// Busca una transacción específica por ID.
    fn find_by_id(&self, id: TransactionId) -> Result<Option<Transaction>, TransactionError>;
    /// Obtiene el historial de transacciones vinculado a una cuenta.
    fn find_for_account(&self, account_id: AccountId)
        -> Result<Vec<Transaction>, TransactionError>;
    /// Registra una nueva transacción.
    fn insert(&self, tx: &Transaction) -> Result<Transaction, TransactionError>;
    /// Actualiza una transacción existente.
    fn update(&self, tx: &Transaction) -> Result<(), TransactionError>;
    /// Actualización selectiva de campos.
    fn update_partial(
        &self,
        id: TransactionId,
        update: TransactionUpdate,
    ) -> Result<(), TransactionError>;
    /// Elimina una transacción.
    fn delete(&self, id: TransactionId) -> Result<(), TransactionError>;
}

/// Representa una parte de una transacción dividida entre varias categorías.
#[derive(uniffi::Record, Debug, Clone, Serialize, Deserialize)]
pub struct SplitTransaction {
    /// ID del registro de split.
    pub id: i64, // SPLITTRANSID
    /// Referencia a la transacción padre.
    pub transaction_id: TransactionId, // TRANSID
    /// Categoría asignada a esta parte.
    pub category_id: Option<CategoryId>, // CATEGID
    /// Monto asignado a esta categoría.
    pub amount: Money, // SPLITTRANSAMOUNT
    /// Comentarios específicos para este split.
    pub notes: Option<String>,
}

/// Interfaz para la persistencia de transacciones divididas.
pub trait SplitRepository {
    /// Recupera los splits asociados a una transacción.
    fn find_for_transaction(
        &self,
        tx_id: TransactionId,
    ) -> Result<Vec<SplitTransaction>, TransactionError>;
    /// Inserta un nuevo split.
    fn insert(&self, split: &SplitTransaction) -> Result<SplitTransaction, TransactionError>;
    /// Actualiza un split existente.
    fn update(&self, split: &SplitTransaction) -> Result<(), TransactionError>;
    /// Elimina un split por ID.
    fn delete(&self, id: i64) -> Result<(), TransactionError>;
    /// Elimina todos los splits asociados a una transacción.
    fn delete_for_transaction(&self, tx_id: TransactionId) -> Result<(), TransactionError>;
}

impl From<TransactionError> for MmexError {
    fn from(e: TransactionError) -> Self {
        match e {
            TransactionError::Common(c) => c,
            _ => MmexError::Internal(e.to_string()),
        }
    }
}
