use crate::api::MmexContext;
use crate::domain::tags::{Tag, TagId};
use crate::domain::transactions::{SplitTransaction, Transaction, TransactionId};
use crate::MmexError;
use std::sync::{Arc, Mutex};

/// Gestor especializado en la administración de transacciones, desgloses y etiquetas vinculadas.
#[derive(uniffi::Object)]
pub struct TransactionManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl TransactionManager {
    /// Obtiene la lista completa de transacciones.
    pub fn get_all(&self) -> Result<Vec<Transaction>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.transactions().get_all_transactions()?)
    }

    /// Busca una transacción específica por su identificador único.
    pub fn get_by_id(&self, id: i64) -> Result<Option<Transaction>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx
            .transactions()
            .get_transaction_by_id(TransactionId { v1: id })?)
    }

    /// Crea una nueva transacción en la base de datos.
    pub fn create(&self, transaction: Transaction) -> Result<Transaction, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.transactions().create_transaction(&transaction)?)
    }

    /// Actualiza la información de una transacción existente.
    pub fn update(&self, transaction: Transaction) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.transactions().update_transaction(&transaction)?;
        Ok(())
    }

    /// Elimina una transacción y sus vínculos asociados (etiquetas y desgloses).
    pub fn delete(&self, id: i64) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.transactions()
            .delete_transaction(TransactionId { v1: id })?;
        Ok(())
    }

    /// Obtiene las etiquetas vinculadas a una transacción específica.
    pub fn get_tags(&self, transaction_id: i64) -> Result<Vec<Tag>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx
            .transactions()
            .get_tags_for_transaction(TransactionId { v1: transaction_id })?)
    }

    /// Vincula una etiqueta a una transacción.
    pub fn link_tag(&self, transaction_id: i64, tag_id: i64) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.transactions()
            .link_tag(TransactionId { v1: transaction_id }, TagId { v1: tag_id })?;
        Ok(())
    }

    /// Desvincula una etiqueta de una transacción.
    pub fn unlink_tag(&self, transaction_id: i64, tag_id: i64) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.transactions()
            .unlink_tag(TransactionId { v1: transaction_id }, TagId { v1: tag_id })?;
        Ok(())
    }

    /// Obtiene los desgloses (splits) asociados a una transacción.
    pub fn get_splits(&self, transaction_id: i64) -> Result<Vec<SplitTransaction>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx
            .transactions()
            .get_splits_for_transaction(TransactionId { v1: transaction_id })?)
    }

    /// Añade un nuevo desglose a una transacción.
    pub fn add_split(&self, split: SplitTransaction) -> Result<SplitTransaction, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.transactions().add_split(&split)?)
    }

    /// Actualiza la información de un desglose existente.
    pub fn update_split(&self, split: SplitTransaction) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.transactions().update_split(&split)?;
        Ok(())
    }

    /// Elimina un desglose por su identificador único.
    pub fn delete_split(&self, split_id: i64) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.transactions().delete_split(split_id)?;
        Ok(())
    }

    /// Obtiene todas las transacciones en formato JSON.
    pub fn get_all_json(&self) -> Result<String, MmexError> {
        let transactions = self.get_all()?;
        serde_json::to_string(&transactions).map_err(|e| MmexError::Internal(e.to_string()))
    }

    /// Obtiene los desgloses de una transacción en formato JSON.
    pub fn get_splits_json(&self, transaction_id: i64) -> Result<String, MmexError> {
        let splits = self.get_splits(transaction_id)?;
        serde_json::to_string(&splits).map_err(|e| MmexError::Internal(e.to_string()))
    }
}
