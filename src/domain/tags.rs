use serde::{Deserialize, Serialize};
use crate::error::MmexError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagId(pub i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
}

pub trait TagRepository {
    fn find_all(&self) -> Result<Vec<Tag>, MmexError>;
    fn find_by_id(&self, id: TagId) -> Result<Option<Tag>, MmexError>;
    fn insert(&self, name: &str) -> Result<Tag, MmexError>;
}
