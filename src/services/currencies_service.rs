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

    pub fn create_currency(&self, currency: &Currency) -> Result<Currency, MmexError> {
        if currency.name.trim().is_empty() || currency.symbol.trim().is_empty() {
            return Err(MmexError::Validation("Currency name and symbol are required".into()));
        }
        let repo = SqlCurrencyRepository::new(self.conn);
        repo.insert(currency)
    }

    pub fn update_currency(&self, currency: &Currency) -> Result<(), MmexError> {
        let repo = SqlCurrencyRepository::new(self.conn);
        repo.update(currency)
    }

    pub fn delete_currency(&self, id: CurrencyId) -> Result<(), MmexError> {
        let repo = SqlCurrencyRepository::new(self.conn);
        repo.delete(id)
    }
}
