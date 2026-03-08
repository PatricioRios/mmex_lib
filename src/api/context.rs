use crate::MmexError;
use crate::services::{
    AccountService, AssetService, CategoryService, CurrencyService, PayeeService, ScheduledService,
    StockService, SupportService, TagService, TransactionService,
};
use rusqlite::Connection;
use std::path::Path;

pub struct MmexContext {
    pub(crate) conn: Connection,
}

impl MmexContext {
    pub fn open(path: &Path, key: Option<String>) -> Result<Self, MmexError> {
        let conn = Connection::open(Path::new(path))?;

        if let Some(password) = key {
            conn.pragma_update(None, "key", password)?;
        }
        Ok(Self { conn })
    }

    pub fn open_memory() -> Result<Self, MmexError> {
        let conn = Connection::open_in_memory()?;
        Ok(Self { conn })
    }

    pub fn accounts(&self) -> AccountService<'_> {
        AccountService::new(&self.conn)
    }

    pub fn tags(&self) -> TagService<'_> {
        TagService::new(&self.conn)
    }

    pub fn payees(&self) -> PayeeService<'_> {
        PayeeService::new(&self.conn)
    }

    pub fn currencies(&self) -> CurrencyService<'_> {
        CurrencyService::new(&self.conn)
    }

    pub fn categories(&self) -> CategoryService<'_> {
        CategoryService::new(&self.conn)
    }

    pub fn transactions(&self) -> TransactionService<'_> {
        TransactionService::new(&self.conn)
    }

    pub fn scheduled(&self) -> ScheduledService<'_> {
        ScheduledService::new(&self.conn)
    }

    pub fn assets(&self) -> AssetService<'_> {
        AssetService::new(&self.conn)
    }

    pub fn stocks(&self) -> StockService<'_> {
        StockService::new(&self.conn)
    }

    pub fn support(&self) -> SupportService<'_> {
        SupportService::new(&self.conn)
    }

    pub fn execute_setup(&self, sql: &str) -> Result<(), MmexError> {
        self.conn.execute_batch(sql).map_err(MmexError::from)
    }
}
