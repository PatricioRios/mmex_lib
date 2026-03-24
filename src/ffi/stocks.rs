use crate::api::MmexContext;
use crate::domain::stocks::{Stock, StockError, StockId, StockUpdate};
use crate::MmexError;
use std::sync::{Arc, Mutex};

/// Gestor especializado en la administración de acciones y valores (Stocks).
#[derive(uniffi::Object)]
pub struct StockManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl StockManager {
    /// Obtiene la lista completa de acciones/valores registrados.
    pub fn get_all(&self) -> Result<Vec<Stock>, StockError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| StockError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.stocks().get_all_stocks()?)
    }

    /// Busca un valor específico por su identificador único.
    pub fn get_by_id(&self, id: i64) -> Result<Option<Stock>, StockError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| StockError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.stocks().get_stock_by_id(StockId { v1: id })?)
    }

    /// Registra un nuevo valor en la base de datos.
    pub fn create(&self, stock: Stock) -> Result<Stock, StockError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| StockError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.stocks().create_stock(&stock)?)
    }

    /// Actualiza la información de un valor existente.
    pub fn update(&self, stock: Stock) -> Result<(), StockError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| StockError::Common(MmexError::Internal(e.to_string())))?;
        ctx.stocks().update_stock(&stock)?;
        Ok(())
    }

    /// Actualiza parcialmente la información de un valor.
    pub fn update_partial(&self, id: i64, update: StockUpdate) -> Result<(), StockError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| StockError::Common(MmexError::Internal(e.to_string())))?;
        ctx.stocks()
            .update_stock_partial(StockId { v1: id }, update)?;
        Ok(())
    }

    /// Elimina un valor de la base de datos.
    pub fn delete(&self, id: i64) -> Result<(), StockError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| StockError::Common(MmexError::Internal(e.to_string())))?;
        ctx.stocks().delete_stock(StockId { v1: id })?;
        Ok(())
    }

    /// Obtiene todos los valores en formato JSON.
    pub fn get_all_json(&self) -> Result<String, StockError> {
        let stocks = self.get_all()?;
        serde_json::to_string(&stocks)
            .map_err(|e| StockError::Common(MmexError::Internal(e.to_string())))
    }
}
