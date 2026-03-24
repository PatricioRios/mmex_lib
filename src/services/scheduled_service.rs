use crate::domain::scheduled_transactions::{
    ScheduledError, ScheduledRepository, ScheduledTransaction,
};
use crate::infrastructure::scheduled_repository::SqlScheduledRepository;
use rusqlite::Connection;

pub struct ScheduledService<'a> {
    conn: &'a Connection,
}

impl<'a> ScheduledService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_scheduled(&self) -> Result<Vec<ScheduledTransaction>, ScheduledError> {
        let repo = SqlScheduledRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_scheduled_by_id(
        &self,
        id: i64,
    ) -> Result<Option<ScheduledTransaction>, ScheduledError> {
        let repo = SqlScheduledRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_scheduled(
        &self,
        tx: &ScheduledTransaction,
    ) -> Result<ScheduledTransaction, ScheduledError> {
        let repo = SqlScheduledRepository::new(self.conn);
        repo.insert(tx)
    }

    pub fn update_scheduled(&self, tx: &ScheduledTransaction) -> Result<(), ScheduledError> {
        let repo = SqlScheduledRepository::new(self.conn);
        repo.update(tx)
    }

    pub fn update_scheduled_partial(
        &self,
        id: i64,
        update: crate::domain::scheduled_transactions::ScheduledUpdate,
    ) -> Result<(), ScheduledError> {
        let repo = SqlScheduledRepository::new(self.conn);
        repo.update_partial(id, update)
    }

    pub fn delete_scheduled(&self, id: i64) -> Result<(), ScheduledError> {
        let repo = SqlScheduledRepository::new(self.conn);
        repo.delete(id)
    }
}
