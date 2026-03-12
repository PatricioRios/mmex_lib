use crate::api::MmexContext;
use crate::MmexError;
use std::sync::{Arc, Mutex};

/// Utilidades de soporte y metadatos del sistema.
#[derive(uniffi::Object)]
pub struct SupportManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl SupportManager {
    /// Obtiene la versión del esquema de la base de datos actual.
    pub fn get_db_version(&self) -> Result<String, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.support().get_db_version()?)
    }
}
