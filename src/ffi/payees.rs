use crate::api::MmexContext;
use crate::domain::payees::{Payee, PayeeError, PayeeId};
use crate::MmexError;
use std::sync::{Arc, Mutex};

/// Gestor especializado en la administración de beneficiarios (Payees).
#[derive(uniffi::Object)]
pub struct PayeeManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl PayeeManager {
    /// Obtiene la lista completa de beneficiarios registrados.
    pub fn get_all(&self) -> Result<Vec<Payee>, PayeeError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| PayeeError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.payees().get_all_payees()?)
    }

    /// Busca un beneficiario específico por su identificador único.
    pub fn get_by_id(&self, id: i64) -> Result<Option<Payee>, PayeeError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| PayeeError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.payees().get_payee_by_id(PayeeId { v1: id })?)
    }

    /// Crea un nuevo beneficiario con el nombre proporcionado.
    pub fn create(&self, name: String) -> Result<Payee, PayeeError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| PayeeError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.payees().create_payee(&name)?)
    }

    /// Actualiza la información de un beneficiario existente.
    pub fn update(&self, payee: Payee) -> Result<(), PayeeError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| PayeeError::Common(MmexError::Internal(e.to_string())))?;
        ctx.payees().update_payee(&payee)?;
        Ok(())
    }

    /// Elimina un beneficiario de la base de datos.
    pub fn delete(&self, id: i64) -> Result<(), PayeeError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| PayeeError::Common(MmexError::Internal(e.to_string())))?;
        ctx.payees().delete_payee(PayeeId { v1: id })?;
        Ok(())
    }

    /// Obtiene todos los beneficiarios en formato JSON.
    pub fn get_all_json(&self) -> Result<String, PayeeError> {
        let payees = self.get_all()?;
        serde_json::to_string(&payees)
            .map_err(|e| PayeeError::Common(MmexError::Internal(e.to_string())))
    }
}
