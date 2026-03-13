use crate::domain::categories::CategoryId;
use crate::domain::payees::PayeeId;
use crate::domain::transactions::{TransactionCode, TransactionStatus};
use crate::domain::types::{AccountId, MmexDate, Money};
use crate::MmexError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScheduledError {
    #[error("Scheduled common error: {0}")]
    Common(#[from] MmexError),

    #[error("Scheduled transaction not found: {0}")]
    NotFound(i64),
}

#[derive(uniffi::Record, Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTransaction {
    pub id: i64, // BDID
    pub account_id: AccountId,
    pub to_account_id: Option<AccountId>,
    pub payee_id: PayeeId,
    pub trans_code: TransactionCode,
    pub amount: Money,
    pub status: TransactionStatus,
    pub transaction_number: Option<String>,
    pub notes: Option<String>,
    pub category_id: Option<CategoryId>,
    pub trans_date: Option<MmexDate>,
    pub next_occurrence_date: Option<MmexDate>,
    pub repeats: i32,
    pub num_occurrences: i32,
    pub to_trans_amount: Option<Money>,
}

pub trait ScheduledRepository {
    fn find_all(&self) -> Result<Vec<ScheduledTransaction>, ScheduledError>;
    fn find_by_id(&self, id: i64) -> Result<Option<ScheduledTransaction>, ScheduledError>;
    fn insert(&self, tx: &ScheduledTransaction) -> Result<ScheduledTransaction, ScheduledError>;
    fn update(&self, tx: &ScheduledTransaction) -> Result<(), ScheduledError>;
    fn delete(&self, id: i64) -> Result<(), ScheduledError>;
}

impl From<ScheduledError> for MmexError {
    fn from(e: ScheduledError) -> Self {
        match e {
            ScheduledError::Common(c) => c,
            _ => MmexError::Internal(e.to_string()),
        }
    }
}
