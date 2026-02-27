use rusqlite::Connection;
use crate::domain::transactions::{Transaction, TransactionId, TransactionRepository, SplitTransaction, SplitRepository};
use crate::domain::tags::{Tag, TagId, TagRepository};
use crate::infrastructure::transactions_repository::SqlTransactionRepository;
use crate::infrastructure::tags_repository::SqlTagRepository;
use crate::infrastructure::splits_repository::SqlSplitRepository;
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
        // 1. Eliminar vínculos de tags
        let tag_repo = SqlTagRepository::new(self.conn);
        let tags = tag_repo.find_for_reference("Transaction", id.0)?;
        for tag in tags {
            tag_repo.unlink_from_reference("Transaction", id.0, tag.id)?;
        }
        
        // 2. Eliminar desgloses (splits)
        let split_repo = SqlSplitRepository::new(self.conn);
        split_repo.delete_for_transaction(id)?;

        // 3. Eliminar transacción
        let repo = SqlTransactionRepository::new(self.conn);
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

    // Gestión de Splits (Desgloses)
    pub fn get_splits_for_transaction(&self, tx_id: TransactionId) -> Result<Vec<SplitTransaction>, MmexError> {
        let repo = SqlSplitRepository::new(self.conn);
        repo.find_for_transaction(tx_id)
    }

    pub fn add_split(&self, split: &SplitTransaction) -> Result<SplitTransaction, MmexError> {
        let repo = SqlSplitRepository::new(self.conn);
        repo.insert(split)
    }

    pub fn update_split(&self, split: &SplitTransaction) -> Result<(), MmexError> {
        let repo = SqlSplitRepository::new(self.conn);
        repo.update(split)
    }

    pub fn delete_split(&self, split_id: i64) -> Result<(), MmexError> {
        let repo = SqlSplitRepository::new(self.conn);
        repo.delete(split_id)
    }
}
