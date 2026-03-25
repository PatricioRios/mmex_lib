use crate::domain::accounts::{
    Account, AccountBalance, AccountError, AccountId, AccountRepository,
};
use crate::domain::transactions::{TransactionCode, TransactionError, TransactionRepository};
use crate::domain::types::Money;
use crate::infrastructure::repositories::SqlAccountRepository;
use crate::infrastructure::transactions_repository::SqlTransactionRepository;
use crate::MmexError;
use rusqlite::Connection;
use rust_decimal::Decimal;

/// Servicio encargado de orquestar la lógica de negocio para las cuentas financieras.
pub struct AccountService<'a> {
    conn: &'a Connection,
}

impl<'a> AccountService<'a> {
    /// Crea una nueva instancia del servicio con una conexión activa.
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Calcula el balance financiero detallado de una cuenta específica.
    /// Suma todos los depósitos y resta los retiros asociados a la cuenta,
    /// incluyendo transferencias entrantes y salientes.
    pub fn get_account_balance(&self, id: AccountId) -> Result<AccountBalance, AccountError> {
        let account_repo = SqlAccountRepository::new(self.conn);
        let account = account_repo
            .find_by_id(id)?
            .ok_or_else(|| AccountError::NotFound(id))?;

        let tx_repo = SqlTransactionRepository::new(self.conn);
        let all_txs = tx_repo.find_all().map_err(|e| match e {
            TransactionError::Common(ce) => AccountError::Common(ce),
            _ => AccountError::Common(MmexError::Internal(e.to_string())),
        })?;

        let mut deposits = Decimal::ZERO;
        let mut withdrawals = Decimal::ZERO;

        for tx in all_txs {
            if tx.account_id == id {
                let amount = tx.amount.to_decimal();
                match tx.trans_code {
                    TransactionCode::Deposit => deposits += amount,
                    TransactionCode::Withdrawal => withdrawals += amount,
                    TransactionCode::Transfer => withdrawals += amount,
                    _ => {}
                }
            } else if tx.to_account_id == Some(id) {
                if tx.trans_code == TransactionCode::Transfer {
                    let incoming = tx
                        .to_amount
                        .as_ref()
                        .map(|m| m.to_decimal())
                        .unwrap_or(tx.amount.to_decimal());
                    deposits += incoming;
                }
            }
        }

        let current = account.initial_balance.to_decimal() + deposits - withdrawals;

        Ok(AccountBalance {
            account_id: id,
            initial_balance: account.initial_balance,
            total_deposits: Money::from(deposits),
            total_withdrawals: Money::from(withdrawals),
            current_balance: Money::from(current),
        })
    }

    /// Obtiene la lista completa de todas las cuentas registradas.
    pub fn get_all_accounts(&self) -> Result<Vec<Account>, AccountError> {
        let repo = SqlAccountRepository::new(self.conn);
        repo.find_all()
    }

    /// Busca una cuenta por su identificador único.
    pub fn get_account_by_id(&self, id: AccountId) -> Result<Option<Account>, AccountError> {
        let repo = SqlAccountRepository::new(self.conn);
        repo.find_by_id(id)
    }

    /// Registra una nueva cuenta validando que el nombre sea obligatorio.
    pub fn create_account(&self, account: &Account) -> Result<Account, AccountError> {
        if account.name.trim().is_empty() {
            return Err(AccountError::NameRequired);
        }
        let repo = SqlAccountRepository::new(self.conn);
        repo.insert(account)
    }

    /// Modifica una cuenta existente de forma completa.
    pub fn update_account(&self, account: &Account) -> Result<(), AccountError> {
        let repo = SqlAccountRepository::new(self.conn);
        repo.update(account)
    }

    /// Modifica una cuenta de forma parcial (solo campos especificados).
    pub fn update_account_partial(
        &self,
        id: AccountId,
        update: crate::domain::accounts::AccountUpdate,
    ) -> Result<(), AccountError> {
        let repo = SqlAccountRepository::new(self.conn);
        repo.update_partial(id, update)
    }

    /// Elimina una cuenta del sistema de forma permanente.
    pub fn delete_account(&self, id: AccountId) -> Result<(), AccountError> {
        let repo = SqlAccountRepository::new(self.conn);
        repo.delete(id)
    }
}
