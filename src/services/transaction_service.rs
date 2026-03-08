use crate::domain::tags::{Tag, TagError, TagId, TagRepository};
use crate::domain::transactions::{
    SplitRepository, SplitTransaction, Transaction, TransactionError, TransactionId,
    TransactionRepository,
};
use crate::infrastructure::splits_repository::SqlSplitRepository;
use crate::infrastructure::tags_repository::SqlTagRepository;
use crate::infrastructure::transactions_repository::SqlTransactionRepository;
use rusqlite::Connection;

pub struct TransactionService<'a> {
    conn: &'a Connection,
}

impl<'a> TransactionService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_transactions(&self) -> Result<Vec<Transaction>, TransactionError> {
        let repo = SqlTransactionRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_transaction_by_id(
        &self,
        id: TransactionId,
    ) -> Result<Option<Transaction>, TransactionError> {
        let repo = SqlTransactionRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_transaction(&self, tx: &Transaction) -> Result<Transaction, TransactionError> {
        let repo = SqlTransactionRepository::new(self.conn);
        repo.insert(tx)
    }

    pub fn update_transaction(&self, tx: &Transaction) -> Result<(), TransactionError> {
        let repo = SqlTransactionRepository::new(self.conn);
        repo.update(tx)
    }

    pub fn delete_transaction(&self, id: TransactionId) -> Result<(), TransactionError> {
        // 1. Eliminar vínculos de tags
        let tag_repo = SqlTagRepository::new(self.conn);
        let tags = tag_repo
            .find_for_reference("Transaction", id.0)
            .map_err(|e| match e {
                TagError::Common(ce) => TransactionError::Common(ce),
                _ => TransactionError::SplitError(e.to_string()),
            })?;
        for tag in tags {
            tag_repo
                .unlink_from_reference("Transaction", id.0, tag.id)
                .map_err(|e| match e {
                    TagError::Common(ce) => TransactionError::Common(ce),
                    _ => TransactionError::SplitError(e.to_string()),
                })?;
        }

        // 2. Eliminar desgloses (splits)
        let split_repo = SqlSplitRepository::new(self.conn);
        split_repo.delete_for_transaction(id)?;

        // 3. Eliminar transacción
        let repo = SqlTransactionRepository::new(self.conn);
        repo.delete(id)
    }

    // Gestión de Tags vinculados
    pub fn get_tags_for_transaction(
        &self,
        id: TransactionId,
    ) -> Result<Vec<Tag>, TransactionError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.find_for_reference("Transaction", id.0)
            .map_err(|e| match e {
                TagError::Common(ce) => TransactionError::Common(ce),
                _ => TransactionError::SplitError(e.to_string()),
            })
    }

    pub fn link_tag(&self, tx_id: TransactionId, tag_id: TagId) -> Result<(), TransactionError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.link_to_reference("Transaction", tx_id.0, tag_id)
            .map_err(|e| match e {
                TagError::Common(ce) => TransactionError::Common(ce),
                _ => TransactionError::SplitError(e.to_string()),
            })
    }

    pub fn unlink_tag(&self, tx_id: TransactionId, tag_id: TagId) -> Result<(), TransactionError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.unlink_from_reference("Transaction", tx_id.0, tag_id)
            .map_err(|e| match e {
                TagError::Common(ce) => TransactionError::Common(ce),
                _ => TransactionError::SplitError(e.to_string()),
            })
    }

    // Gestión de Splits (Desgloses)
    pub fn get_splits_for_transaction(
        &self,
        tx_id: TransactionId,
    ) -> Result<Vec<SplitTransaction>, TransactionError> {
        let repo = SqlSplitRepository::new(self.conn);
        repo.find_for_transaction(tx_id)
    }

    pub fn add_split(
        &self,
        split: &SplitTransaction,
    ) -> Result<SplitTransaction, TransactionError> {
        let repo = SqlSplitRepository::new(self.conn);
        repo.insert(split)
    }

    pub fn update_split(&self, split: &SplitTransaction) -> Result<(), TransactionError> {
        let repo = SqlSplitRepository::new(self.conn);
        repo.update(split)
    }

    pub fn delete_split(&self, split_id: i64) -> Result<(), TransactionError> {
        let repo = SqlSplitRepository::new(self.conn);
        repo.delete(split_id)
    }
}
