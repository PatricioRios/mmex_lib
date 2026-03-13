use crate::api::MmexContext;
use crate::domain::accounts::{Account, AccountBalance, AccountError};
use crate::domain::types::AccountId;
use crate::MmexError;
use std::sync::{Arc, Mutex};

/// Gestor especializado en la administración de cuentas bancarias y balances.
#[derive(uniffi::Object)]
pub struct AccountManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl AccountManager {
    /// Obtiene la lista completa de cuentas registradas.
    pub fn get_all(&self) -> Result<Vec<Account>, AccountError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| AccountError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.accounts().get_all_accounts()?)
    }

    /// Busca una cuenta específica por su identificador único.
    pub fn get_by_id(&self, id: i64) -> Result<Option<Account>, AccountError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| AccountError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.accounts().get_account_by_id(AccountId { v1: id })?)
    }

    /// Crea una nueva cuenta en la base de datos.
    pub fn create(&self, account: Account) -> Result<Account, AccountError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| AccountError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.accounts().create_account(&account)?)
    }

    /// Actualiza la información de una cuenta existente.
    pub fn update(&self, account: Account) -> Result<(), AccountError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| AccountError::Common(MmexError::Internal(e.to_string())))?;
        ctx.accounts().update_account(&account)?;
        Ok(())
    }

    /// Elimina una cuenta de la base de datos.
    pub fn delete(&self, id: i64) -> Result<(), AccountError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| AccountError::Common(MmexError::Internal(e.to_string())))?;
        ctx.accounts().delete_account(AccountId { v1: id })?;
        Ok(())
    }

    /// Calcula el balance financiero detallado de una cuenta (depósitos, retiros y saldo actual).
    pub fn get_balance(&self, account_id: i64) -> Result<AccountBalance, AccountError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| AccountError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx
            .accounts()
            .get_account_balance(AccountId { v1: account_id })?)
    }

    /// Obtiene todas las cuentas en formato JSON.
    pub fn get_all_json(&self) -> Result<String, AccountError> {
        let accounts = self.get_all()?;
        serde_json::to_string(&accounts)
            .map_err(|e| AccountError::Common(MmexError::Internal(e.to_string())))
    }

    /// Obtiene el balance de una cuenta en formato JSON.
    pub fn get_balance_json(&self, account_id: i64) -> Result<String, AccountError> {
        let balance = self.get_balance(account_id)?;
        serde_json::to_string(&balance)
            .map_err(|e| AccountError::Common(MmexError::Internal(e.to_string())))
    }
}
