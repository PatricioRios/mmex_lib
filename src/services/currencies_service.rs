use rusqlite::Connection;
use crate::domain::currencies::{Currency, CurrencyId, CurrencyRepository};
use crate::infrastructure::currencies_repository::SqlCurrencyRepository;
use crate::error::MmexError;

pub struct CurrencyService<'a> {
    conn: &'a Connection,
}

impl<'a> CurrencyService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_currencies(&self) -> Result<Vec<Currency>, MmexError> {
        let repo = SqlCurrencyRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_currency_by_id(&self, id: CurrencyId) -> Result<Option<Currency>, MmexError> {
        let repo = SqlCurrencyRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn get_currency_by_symbol(&self, symbol: &str) -> Result<Option<Currency>, MmexError> {
        let repo = SqlCurrencyRepository::new(self.conn);
        repo.find_by_symbol(symbol)
    }
}
