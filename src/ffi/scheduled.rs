use crate::api::MmexContext;
use crate::domain::scheduled_transactions::ScheduledTransaction;
use crate::MmexError;
use std::sync::{Arc, Mutex};

/// Gestor especializado en la administración de transacciones programadas (Scheduled).
#[derive(uniffi::Object)]
pub struct ScheduledManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl ScheduledManager {
    /// Obtiene la lista completa de transacciones programadas.
    pub fn get_all(&self) -> Result<Vec<ScheduledTransaction>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.scheduled().get_all_scheduled()?)
    }

    /// Busca una transacción programada específica por su identificador único.
    pub fn get_by_id(&self, id: i64) -> Result<Option<ScheduledTransaction>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.scheduled().get_scheduled_by_id(id)?)
    }

    /// Crea una nueva transacción programada.
    pub fn create(
        &self,
        transaction: ScheduledTransaction,
    ) -> Result<ScheduledTransaction, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.scheduled().create_scheduled(&transaction)?)
    }

    /// Actualiza una transacción programada existente.
    pub fn update(&self, transaction: ScheduledTransaction) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.scheduled().update_scheduled(&transaction)?;
        Ok(())
    }

    /// Elimina una transacción programada de la base de datos.
    pub fn delete(&self, id: i64) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.scheduled().delete_scheduled(id)?;
        Ok(())
    }

    /// Obtiene todas las transacciones programadas en formato JSON.
    pub fn get_all_json(&self) -> Result<String, MmexError> {
        let scheduled = self.get_all()?;
        serde_json::to_string(&scheduled).map_err(|e| MmexError::Internal(e.to_string()))
    }
}
