use crate::api::MmexContext;
use crate::domain::currencies::{Currency, CurrencyError, CurrencyId, CurrencyUpdate};
use crate::MmexError;
use std::sync::{Arc, Mutex};

/// Gestor especializado en la administración de monedas y tipos de cambio.
#[derive(uniffi::Object)]
pub struct CurrencyManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl CurrencyManager {
    /// Obtiene la lista completa de monedas registradas.
    pub fn get_all(&self) -> Result<Vec<Currency>, CurrencyError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CurrencyError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.currencies().get_all_currencies()?)
    }

    /// Busca una moneda específica por su identificador único.
    pub fn get_by_id(&self, id: i64) -> Result<Option<Currency>, CurrencyError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CurrencyError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.currencies().get_currency_by_id(CurrencyId { v1: id })?)
    }

    /// Busca una moneda por su símbolo (ej: "USD").
    pub fn get_by_symbol(&self, symbol: String) -> Result<Option<Currency>, CurrencyError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CurrencyError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.currencies().get_currency_by_symbol(&symbol)?)
    }

    /// Crea una nueva moneda en la base de datos.
    pub fn create(&self, currency: Currency) -> Result<Currency, CurrencyError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CurrencyError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.currencies().create_currency(&currency)?)
    }

    /// Actualiza la información de una moneda existente.
    pub fn update(&self, currency: Currency) -> Result<(), CurrencyError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CurrencyError::Common(MmexError::Internal(e.to_string())))?;
        ctx.currencies().update_currency(&currency)?;
        Ok(())
    }

    /// Actualiza parcialmente la información de una moneda.
    pub fn update_partial(&self, id: i64, update: CurrencyUpdate) -> Result<(), CurrencyError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CurrencyError::Common(MmexError::Internal(e.to_string())))?;
        ctx.currencies()
            .update_currency_partial(CurrencyId { v1: id }, update)?;
        Ok(())
    }

    /// Elimina una moneda de la base de datos.
    pub fn delete(&self, id: i64) -> Result<(), CurrencyError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CurrencyError::Common(MmexError::Internal(e.to_string())))?;
        ctx.currencies().delete_currency(CurrencyId { v1: id })?;
        Ok(())
    }

    /// Obtiene todas las monedas en formato JSON.
    pub fn get_all_json(&self) -> Result<String, CurrencyError> {
        let currencies = self.get_all()?;
        serde_json::to_string(&currencies)
            .map_err(|e| CurrencyError::Common(MmexError::Internal(e.to_string())))
    }
}
