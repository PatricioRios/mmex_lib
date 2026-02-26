use serde::{Deserialize, Serialize};
pub use crate::domain::types::{AccountId, Money};
use crate::domain::currencies::CurrencyId;
use crate::error::MmexError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountType {
    Cash, Checking, Term, Investment, CreditCard, Loan, Asset, Shares, Unknown(String),
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
    Open, Closed, Unknown(String),
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

pub trait AccountRepository {
    fn find_all(&self) -> Result<Vec<Account>, MmexError>;
    fn find_by_id(&self, id: AccountId) -> Result<Option<Account>, MmexError>;
    fn insert(&self, account: &Account) -> Result<Account, MmexError>;
    fn update(&self, account: &Account) -> Result<(), MmexError>;
    fn delete(&self, id: AccountId) -> Result<(), MmexError>;
}
