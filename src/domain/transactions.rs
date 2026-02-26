use serde::{Deserialize, Serialize};
pub use crate::domain::types::{AccountId, Money, TransactionId};
use crate::domain::payees::PayeeId;
use crate::domain::categories::CategoryId;
use chrono::NaiveDate;
use crate::error::MmexError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionCode {
    Withdrawal, Deposit, Transfer, Unknown(String),
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
    None, Reconciled, Void, FollowUp, Duplicate, Unknown(String),
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
            Self::None => "None".to_string(),
            Self::Reconciled => "Reconciled".to_string(),
            Self::Void => "Void".to_string(),
            Self::FollowUp => "Follow up".to_string(),
            Self::Duplicate => "Duplicate".to_string(),
            Self::Unknown(s) => s.clone(),
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
    pub id: i64, // SPLITTRANSID
    pub transaction_id: TransactionId, // TRANSID
    pub category_id: Option<CategoryId>, // CATEGID
    pub amount: Money, // SPLITTRANSAMOUNT
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
    fn find_for_transaction(&self, tx_id: TransactionId) -> Result<Vec<SplitTransaction>, MmexError>;
    fn insert(&self, split: &SplitTransaction) -> Result<SplitTransaction, MmexError>;
    fn update(&self, split: &SplitTransaction) -> Result<(), MmexError>;
    fn delete(&self, id: i64) -> Result<(), MmexError>;
    fn delete_for_transaction(&self, tx_id: TransactionId) -> Result<(), MmexError>;
}
