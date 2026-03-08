pub use crate::domain::types::TagId;
use crate::MmexError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
pub enum TagError {
    #[error("Tag common error: {0}")]
    Common(#[from] MmexError),

    #[error("Tag not found: {0}")]
    NotFound(TagId),

    #[error("Tag name is required")]
    NameRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
}

pub trait TagRepository {
    fn find_all(&self) -> Result<Vec<Tag>, TagError>;
    fn find_by_id(&self, id: TagId) -> Result<Option<Tag>, TagError>;
    fn insert(&self, name: &str) -> Result<Tag, TagError>;
    fn update(&self, tag: &Tag) -> Result<(), TagError>;
    fn delete(&self, id: TagId) -> Result<(), TagError>;

    // Gestión de vínculos (TAGLINK_V1)
    fn find_for_reference(&self, ref_type: &str, ref_id: i64) -> Result<Vec<Tag>, TagError>;
    fn link_to_reference(&self, ref_type: &str, ref_id: i64, tag_id: TagId)
        -> Result<(), TagError>;
    fn unlink_from_reference(
        &self,
        ref_type: &str,
        ref_id: i64,
        tag_id: TagId,
    ) -> Result<(), TagError>;
}

impl From<TagError> for MmexError {
    fn from(e: TagError) -> Self {
        match e {
            TagError::Common(c) => c,
            _ => MmexError::Internal(e.to_string()),
        }
    }
}
