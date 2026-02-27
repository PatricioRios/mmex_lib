use serde::{Deserialize, Serialize};
use crate::error::MmexError;
pub use crate::domain::types::TagId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
}

pub trait TagRepository {
    fn find_all(&self) -> Result<Vec<Tag>, MmexError>;
    fn find_by_id(&self, id: TagId) -> Result<Option<Tag>, MmexError>;
    fn insert(&self, name: &str) -> Result<Tag, MmexError>;
    fn update(&self, tag: &Tag) -> Result<(), MmexError>;
    fn delete(&self, id: TagId) -> Result<(), MmexError>;
    
    // Gestión de vínculos (TAGLINK_V1)
    fn find_for_reference(&self, ref_type: &str, ref_id: i64) -> Result<Vec<Tag>, MmexError>;
    fn link_to_reference(&self, ref_type: &str, ref_id: i64, tag_id: TagId) -> Result<(), MmexError>;
    fn unlink_from_reference(&self, ref_type: &str, ref_id: i64, tag_id: TagId) -> Result<(), MmexError>;
}
