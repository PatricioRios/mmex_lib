use serde::{Deserialize, Serialize};
use crate::error::MmexError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CategoryId(pub i32);

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
}
