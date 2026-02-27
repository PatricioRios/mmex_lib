use rusqlite::Connection;
use crate::infrastructure::repositories::SqlAccountRepository;
use crate::infrastructure::transactions_repository::SqlTransactionRepository;
use crate::domain::accounts::{Account, AccountId, AccountRepository, AccountBalance};
use crate::domain::transactions::{TransactionCode, TransactionRepository};
use crate::domain::types::Money;
use crate::error::MmexError;
use rust_decimal::Decimal;

pub struct AccountService<'a> {
    conn: &'a Connection,
}

impl<'a> AccountService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_account_balance(&self, id: AccountId) -> Result<AccountBalance, MmexError> {
        let account_repo = SqlAccountRepository::new(self.conn);
        let account = account_repo.find_by_id(id)?
            .ok_or_else(|| MmexError::NotFound(format!("Account with id {} not found", id.0)))?;

        let tx_repo = SqlTransactionRepository::new(self.conn);
        let all_txs = tx_repo.find_all()?;

        let mut deposits = Decimal::ZERO;
        let mut withdrawals = Decimal::ZERO;

        for tx in all_txs {
            if tx.account_id == id {
                match tx.trans_code {
                    TransactionCode::Deposit => deposits += tx.amount.0,
                    TransactionCode::Withdrawal => withdrawals += tx.amount.0,
                    TransactionCode::Transfer => withdrawals += tx.amount.0,
                    _ => {}
                }
            } else if tx.to_account_id == Some(id) {
                if tx.trans_code == TransactionCode::Transfer {
                    let incoming = tx.to_amount.map(|m| m.0).unwrap_or(tx.amount.0);
                    deposits += incoming;
                }
            }
        }

        let current = account.initial_balance.0 + deposits - withdrawals;

        Ok(AccountBalance {
            account_id: id,
            initial_balance: account.initial_balance,
            total_deposits: Money(deposits),
            total_withdrawals: Money(withdrawals),
            current_balance: Money(current),
        })
    }

    pub fn get_all_accounts(&self) -> Result<Vec<Account>, MmexError> {
        let repo = SqlAccountRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_account_by_id(&self, id: AccountId) -> Result<Option<Account>, MmexError> {
        let repo = SqlAccountRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_account(&self, account: &Account) -> Result<Account, MmexError> {
        if account.name.trim().is_empty() {
            return Err(MmexError::Validation("Account name is required".into()));
        }
        let repo = SqlAccountRepository::new(self.conn);
        repo.insert(account)
    }

    pub fn update_account(&self, account: &Account) -> Result<(), MmexError> {
        let repo = SqlAccountRepository::new(self.conn);
        repo.update(account)
    }

    pub fn delete_account(&self, id: AccountId) -> Result<(), MmexError> {
        let repo = SqlAccountRepository::new(self.conn);
        repo.delete(id)
    }
}
