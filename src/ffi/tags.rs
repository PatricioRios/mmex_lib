use crate::api::MmexContext;
use crate::domain::tags::{Tag, TagError};
use crate::domain::types::TagId;
use crate::MmexError;
use std::sync::{Arc, Mutex};

/// Gestor especializado en la administración de etiquetas (Tags).
#[derive(uniffi::Object)]
pub struct TagManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl TagManager {
    /// Obtiene la lista completa de etiquetas registradas.
    pub fn get_all(&self) -> Result<Vec<Tag>, TagError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| TagError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.tags().get_all_tags()?)
    }

    /// Busca una etiqueta específica por su identificador único.
    pub fn get_by_id(&self, id: i64) -> Result<Option<Tag>, TagError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| TagError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.tags().get_tag_by_id(TagId::new(id))?)
    }

    /// Crea una nueva etiqueta con el nombre proporcionado.
    pub fn create(&self, name: String) -> Result<Tag, TagError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| TagError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.tags().create_tag(&name)?)
    }

    /// Actualiza el nombre de una etiqueta existente.
    pub fn update(&self, id: i64, name: String) -> Result<(), TagError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| TagError::Common(MmexError::Internal(e.to_string())))?;
        let tag = Tag {
            id: TagId::new(id),
            name,
        };
        ctx.tags().update_tag(&tag)?;
        Ok(())
    }

    /// Elimina una etiqueta de la base de datos.
    pub fn delete(&self, id: i64) -> Result<(), TagError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| TagError::Common(MmexError::Internal(e.to_string())))?;
        ctx.tags().delete_tag(TagId::new(id))?;
        Ok(())
    }

    /// Obtiene las etiquetas vinculadas a una referencia específica (ej: transacciones).
    pub fn get_for_reference(&self, ref_type: String, ref_id: i64) -> Result<Vec<Tag>, TagError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| TagError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.tags().get_for_reference(&ref_type, ref_id)?)
    }

    /// Vincula una etiqueta a una referencia.
    pub fn link_to_reference(
        &self,
        ref_type: String,
        ref_id: i64,
        tag_id: i64,
    ) -> Result<(), TagError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| TagError::Common(MmexError::Internal(e.to_string())))?;
        ctx.tags()
            .link_to_reference(&ref_type, ref_id, TagId::new(tag_id))?;
        Ok(())
    }

    /// Desvincula una etiqueta de una referencia.
    pub fn unlink_from_reference(
        &self,
        ref_type: String,
        ref_id: i64,
        tag_id: i64,
    ) -> Result<(), TagError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| TagError::Common(MmexError::Internal(e.to_string())))?;
        ctx.tags()
            .unlink_from_reference(&ref_type, ref_id, TagId::new(tag_id))?;
        Ok(())
    }

    /// Obtiene todas las etiquetas en formato JSON (útil para integraciones heredadas).
    pub fn get_all_json(&self) -> Result<String, TagError> {
        let tags = self.get_all()?;
        serde_json::to_string(&tags)
            .map_err(|e| TagError::Common(MmexError::Internal(e.to_string())))
    }
}
