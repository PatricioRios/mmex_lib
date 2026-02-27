use rusqlite::Connection;
use crate::domain::scheduled_transactions::{ScheduledTransaction, ScheduledRepository};
use crate::infrastructure::scheduled_repository::SqlScheduledRepository;
use crate::error::MmexError;

pub struct ScheduledService<'a> {
    conn: &'a Connection,
}

impl<'a> ScheduledService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_scheduled(&self) -> Result<Vec<ScheduledTransaction>, MmexError> {
        let repo = SqlScheduledRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_scheduled_by_id(&self, id: i64) -> Result<Option<ScheduledTransaction>, MmexError> {
        let repo = SqlScheduledRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_scheduled(&self, tx: &ScheduledTransaction) -> Result<ScheduledTransaction, MmexError> {
        let repo = SqlScheduledRepository::new(self.conn);
        repo.insert(tx)
    }

    pub fn update_scheduled(&self, tx: &ScheduledTransaction) -> Result<(), MmexError> {
        let repo = SqlScheduledRepository::new(self.conn);
        repo.update(tx)
    }

    pub fn delete_scheduled(&self, id: i64) -> Result<(), MmexError> {
        let repo = SqlScheduledRepository::new(self.conn);
        repo.delete(id)
    }
}
