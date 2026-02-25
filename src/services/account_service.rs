use rusqlite::Connection;
use crate::infrastructure::repositories::SqlAccountRepository;
use crate::domain::repositories::AccountRepository;
use crate::domain::models::Account;
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
}
