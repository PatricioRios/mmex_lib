use crate::api::MmexContext;
use crate::domain::types::AccountId;
use crate::MmexError;
use std::sync::{Arc, Mutex};

#[derive(uniffi::Object)]
pub struct AccountManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl AccountManager {
    pub fn get_all_json(&self) -> Result<String, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        let accounts = ctx.accounts().get_all_accounts()?;
        serde_json::to_string(&accounts).map_err(|e| MmexError::Internal(e.to_string()))
    }

    pub fn get_balance_json(&self, account_id: i64) -> Result<String, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        let balance = ctx
            .accounts()
            .get_account_balance(AccountId { v1: account_id })?;
        serde_json::to_string(&balance).map_err(|e| MmexError::Internal(e.to_string()))
    }
}
