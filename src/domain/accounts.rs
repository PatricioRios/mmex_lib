pub use crate::domain::types::{AccountId, CurrencyId, Money};
use crate::MmexError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
pub enum AccountError {
    #[error("Account common error: {0}")]
    Common(#[from] MmexError),

    #[error("Account not found: {0}")]
    NotFound(AccountId),

    #[error("Account name is required")]
    NameRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountType {
    Cash,
    Checking,
    Term,
    Investment,
    CreditCard,
    Loan,
    Asset,
    Shares,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountStatus {
    Open,
    Closed,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub name: String,
    pub account_type: AccountType,
    pub account_num: Option<String>,
    pub status: AccountStatus,
    pub notes: Option<String>,
    pub initial_balance: Money,
    pub currency_id: CurrencyId,
    pub favorite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub account_id: AccountId,
    pub initial_balance: Money,
    pub total_deposits: Money,
    pub total_withdrawals: Money,
    pub current_balance: Money,
}

pub trait AccountRepository {
    fn find_all(&self) -> Result<Vec<Account>, AccountError>;
    fn find_by_id(&self, id: AccountId) -> Result<Option<Account>, AccountError>;
    fn insert(&self, account: &Account) -> Result<Account, AccountError>;
    fn update(&self, account: &Account) -> Result<(), AccountError>;
    fn delete(&self, id: AccountId) -> Result<(), AccountError>;
}

impl From<AccountError> for MmexError {
    fn from(e: AccountError) -> Self {
        match e {
            AccountError::Common(c) => c,
            _ => MmexError::Internal(e.to_string()),
        }
    }
}
