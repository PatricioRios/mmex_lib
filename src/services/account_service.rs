use rusqlite::Connection;
use crate::infrastructure::repositories::SqlAccountRepository;
use crate::domain::accounts::{Account, AccountId, AccountRepository};
use crate::error::MmexError;

pub struct AccountService<'a> {
    conn: &'a Connection,
}

impl<'a> AccountService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
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
