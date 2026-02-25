use std::path::Path;
use rusqlite::Connection;
use crate::error::MmexError;
use crate::services::{AccountService, TagService, PayeeService, CurrencyService, CategoryService};

pub struct MmexContext {
    conn: Connection,
}

impl MmexContext {
    pub fn open(path: &Path, key: Option<&str>) -> Result<Self, MmexError> {
        let conn = Connection::open(path)?;

        if let Some(password) = key {
            // SQLCipher PRAGMA key
            conn.pragma_update(None, "key", password)?;
        }
        // Validación básica de que es una DB de MMEX (ej. chequear tabla de metadata)
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

    /// Método para ejecutar scripts de inicialización (útil para tests o migraciones)
    pub fn execute_setup(&self, sql: &str) -> Result<(), MmexError> {
        self.conn.execute_batch(sql).map_err(MmexError::from)
    }
}
