use crate::domain::types::{AccountId, Money};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub name: String,
    pub initial_balance: Money,
    pub account_type: String, // Podría ser un enum más adelante
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: crate::domain::types::TransactionId,
    pub account_id: AccountId,
    pub amount: Money,
    pub date: chrono::NaiveDate,
    pub notes: Option<String>,
}
