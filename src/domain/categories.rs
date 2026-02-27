use serde::{Deserialize, Serialize};
use crate::error::MmexError;
pub use crate::domain::types::CategoryId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
    pub active: bool,
    pub parent_id: Option<CategoryId>, // None si es raíz (-1 en DB)
}

pub trait CategoryRepository {
    fn find_all(&self) -> Result<Vec<Category>, MmexError>;
    fn find_by_id(&self, id: CategoryId) -> Result<Option<Category>, MmexError>;
    fn find_subcategories(&self, parent_id: CategoryId) -> Result<Vec<Category>, MmexError>;
    fn insert(&self, category: &Category) -> Result<Category, MmexError>;
    fn update(&self, category: &Category) -> Result<(), MmexError>;
    fn delete(&self, id: CategoryId) -> Result<(), MmexError>;
}
