use std::path::Path;
use rusqlite::Connection;
use crate::error::MmexError;
use crate::services::{AccountService, TagService, PayeeService, CurrencyService, CategoryService, TransactionService, ScheduledService, AssetService, StockService, SupportService};

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct MmexContext {
    pub(crate) conn: Connection,
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl MmexContext {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub fn open(path: &str, key: Option<String>) -> Result<Self, MmexError> {
        let conn = Connection::open(Path::new(path))?;

        if let Some(password) = key {
            conn.pragma_update(None, "key", password)?;
        }
        Ok(Self { conn })
    }

    pub fn get_db_version(&self) -> Result<String, MmexError> {
        self.support().get_db_version()
    }

    // Nota: Para FFI es mejor devolver tipos simples o Records de UniFFI.
    // Aquí implementaremos wrappers que faciliten la vida al MCP
}

// Wrappers para NAPI-RS (Node.js)
#[cfg(feature = "napi")]
#[napi_derive::napi]
impl MmexContext {
    #[napi_derive::napi(constructor)]
    pub fn napi_open(path: String, key: Option<String>) -> napi::Result<Self> {
        Self::open(&path, key).map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    #[napi_derive::napi]
    pub fn napi_get_accounts(&self) -> napi::Result<String> {
        let accounts = self.accounts().get_all_accounts()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        serde_json::to_string(&accounts).map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}

// Métodos internos de Rust (no exportados a FFI directamente)
impl MmexContext {
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
