use rusqlite::Connection;
use crate::infrastructure::repositories::SqlSupportRepository;
use crate::domain::models::SupportRepository;
use crate::error::MmexError;

pub struct SupportService<'a> {
    conn: &'a Connection,
}

impl<'a> SupportService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_db_version(&self) -> Result<String, MmexError> {
        let repo = SqlSupportRepository::new(self.conn);
        repo.get_metadata("DATAVERSION")?
            .ok_or_else(|| MmexError::Internal("DATAVERSION not found".into()))
    }

    pub fn get_setting(&self, name: &str) -> Result<Option<String>, MmexError> {
        let repo = SqlSupportRepository::new(self.conn);
        repo.get_setting(name)
    }

    pub fn set_setting(&self, name: &str, value: &str) -> Result<(), MmexError> {
        let repo = SqlSupportRepository::new(self.conn);
        repo.set_setting(name, value)
    }
}
