use crate::domain::categories::CategoryId;
use crate::domain::payees::PayeeId;
pub use crate::domain::types::{AccountId, Money, TransactionId};
use crate::error::MmexError;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionCode {
    Withdrawal,
    Deposit,
    Transfer,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    None,
    Reconciled,
    Void,
    FollowUp,
    Duplicate,
    Unknown(String),
}

// TODO: Verificar como la db maneja los status. Por ejemplo "R" es Reconciled.
impl From<String> for TransactionStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "None" | "" => Self::None,
            "Reconciled" | "R" => Self::Reconciled,
            "Void" | "V" => Self::Void,
            "Follow up" | "F" => Self::FollowUp,
            "Duplicate" | "D" => Self::Duplicate,
            _ => Self::Unknown(s),
        }
    }
}

// TODO: Verificar como la db maneja los status. Por ejemplo "R" es Reconciled.
impl ToString for TransactionStatus {
    fn to_string(&self) -> String {
        match self {
            Self::None => "".to_string(),             // En la db es ""
            Self::Reconciled => "R".to_string(), // En la db es "R"
            Self::Void => "V".to_string(),             // En la db es "V"
            Self::FollowUp => "F".to_string(),    // En la db es "F"
            Self::Duplicate => "D".to_string(),   // En la db es "D"
            Self::Unknown(s) => s.clone(),                // En la db es ""
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: TransactionId,
    pub account_id: AccountId,
    pub to_account_id: Option<AccountId>,
    pub payee_id: PayeeId,
    pub trans_code: TransactionCode,
    pub amount: Money,
    pub status: TransactionStatus,
    pub transaction_number: Option<String>,
    pub notes: Option<String>,
    pub category_id: Option<CategoryId>,
    pub date: Option<NaiveDate>,
    pub to_amount: Option<Money>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitTransaction {
    pub id: i64,                         // SPLITTRANSID
    pub transaction_id: TransactionId,   // TRANSID
    pub category_id: Option<CategoryId>, // CATEGID
    pub amount: Money,                   // SPLITTRANSAMOUNT
    pub notes: Option<String>,
}

pub trait TransactionRepository {
    fn find_all(&self) -> Result<Vec<Transaction>, MmexError>;
    fn find_by_id(&self, id: TransactionId) -> Result<Option<Transaction>, MmexError>;
    fn insert(&self, tx: &Transaction) -> Result<Transaction, MmexError>;
    fn update(&self, tx: &Transaction) -> Result<(), MmexError>;
    fn delete(&self, id: TransactionId) -> Result<(), MmexError>;
}

pub trait SplitRepository {
    fn find_for_transaction(
        &self,
        tx_id: TransactionId,
    ) -> Result<Vec<SplitTransaction>, MmexError>;
    fn insert(&self, split: &SplitTransaction) -> Result<SplitTransaction, MmexError>;
    fn update(&self, split: &SplitTransaction) -> Result<(), MmexError>;
    fn delete(&self, id: i64) -> Result<(), MmexError>;
    fn delete_for_transaction(&self, tx_id: TransactionId) -> Result<(), MmexError>;
}
