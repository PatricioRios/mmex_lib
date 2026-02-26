use rusqlite::Connection;
use crate::domain::transactions::{Transaction, TransactionId, TransactionRepository};
use crate::domain::tags::{Tag, TagId, TagRepository};
use crate::infrastructure::transactions_repository::SqlTransactionRepository;
use crate::infrastructure::tags_repository::SqlTagRepository;
use crate::error::MmexError;

pub struct TransactionService<'a> {
    conn: &'a Connection,
}

impl<'a> TransactionService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_transactions(&self) -> Result<Vec<Transaction>, MmexError> {
        let repo = SqlTransactionRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_transaction_by_id(&self, id: TransactionId) -> Result<Option<Transaction>, MmexError> {
        let repo = SqlTransactionRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_transaction(&self, tx: &Transaction) -> Result<Transaction, MmexError> {
        let repo = SqlTransactionRepository::new(self.conn);
        repo.insert(tx)
    }

    pub fn update_transaction(&self, tx: &Transaction) -> Result<(), MmexError> {
        let repo = SqlTransactionRepository::new(self.conn);
        repo.update(tx)
    }

    pub fn delete_transaction(&self, id: TransactionId) -> Result<(), MmexError> {
        let repo = SqlTransactionRepository::new(self.conn);
        // Nota: Deberíamos eliminar también vínculos de tags si no hay integridad referencial
        let tag_repo = SqlTagRepository::new(self.conn);
        let tags = tag_repo.find_for_reference("Transaction", id.0)?;
        for tag in tags {
            tag_repo.unlink_from_reference("Transaction", id.0, tag.id)?;
        }
        repo.delete(id)
    }

    // Gestión de Tags vinculados
    pub fn get_tags_for_transaction(&self, id: TransactionId) -> Result<Vec<Tag>, MmexError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.find_for_reference("Transaction", id.0)
    }

    pub fn link_tag(&self, tx_id: TransactionId, tag_id: TagId) -> Result<(), MmexError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.link_to_reference("Transaction", tx_id.0, tag_id)
    }

    pub fn unlink_tag(&self, tx_id: TransactionId, tag_id: TagId) -> Result<(), MmexError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.unlink_from_reference("Transaction", tx_id.0, tag_id)
    }
}
