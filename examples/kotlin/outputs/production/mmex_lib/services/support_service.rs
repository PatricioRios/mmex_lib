use crate::domain::models::{SupportError, SupportRepository};
use crate::infrastructure::repositories::SqlSupportRepository;
use rusqlite::Connection;

pub struct SupportService<'a> {
    conn: &'a Connection,
}

impl<'a> SupportService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_db_version(&self) -> Result<String, SupportError> {
        let repo = SqlSupportRepository::new(self.conn);
        repo.get_metadata("DATAVERSION")?
            .ok_or_else(|| SupportError::NotFound("DATAVERSION".into()))
    }

    pub fn get_setting(&self, name: &str) -> Result<Option<String>, SupportError> {
        let repo = SqlSupportRepository::new(self.conn);
        repo.get_setting(name)
    }

    pub fn set_setting(&self, name: &str, value: &str) -> Result<(), SupportError> {
        let repo = SqlSupportRepository::new(self.conn);
        repo.set_setting(name, value)
    }
}
