use crate::api::MmexContext;
use crate::domain::tags::Tag;
use crate::domain::types::TagId;
use crate::MmexError;
use std::sync::{Arc, Mutex};

#[derive(uniffi::Object)]
pub struct TagManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl TagManager {
    pub fn get_all(&self) -> Result<Vec<Tag>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.tags().get_all_tags()?)
    }

    pub fn get_by_id(&self, id: i64) -> Result<Option<Tag>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.tags().get_tag_by_id(TagId { v1: id })?)
    }

    pub fn create(&self, name: String) -> Result<Tag, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.tags().create_tag(&name)?)
    }

    pub fn update(&self, id: i64, name: String) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        let tag = Tag {
            id: TagId { v1: id },
            name,
        };
        ctx.tags().update_tag(&tag)?;
        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.tags().delete_tag(TagId { v1: id })?;
        Ok(())
    }

    // JSON versions for compatibility if needed
    pub fn get_all_json(&self) -> Result<String, MmexError> {
        let tags = self.get_all()?;
        serde_json::to_string(&tags).map_err(|e| MmexError::Internal(e.to_string()))
    }
}
