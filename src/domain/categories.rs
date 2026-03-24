pub use crate::domain::types::CategoryId;
use crate::MmexError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(uniffi::Error, Error, Debug)]
pub enum CategoryError {
    #[error("Category common error: {0}")]
    Common(#[from] MmexError),

    #[error("Category not found: {0}")]
    NotFound(CategoryId),

    #[error("Category name is required")]
    NameRequired,
}

#[derive(uniffi::Record, Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
    pub active: bool,
    pub parent_id: Option<CategoryId>, // None si es raíz (-1 en DB)
}

#[derive(uniffi::Record, Debug, Clone, Default)]
pub struct CategoryUpdate {
    pub name: Option<String>,
    pub active: Option<bool>,
    pub parent_id: Option<CategoryId>,
}

pub trait CategoryRepository {
    fn find_all(&self) -> Result<Vec<Category>, CategoryError>;
    fn find_by_id(&self, id: CategoryId) -> Result<Option<Category>, CategoryError>;
    fn find_subcategories(&self, parent_id: CategoryId) -> Result<Vec<Category>, CategoryError>;
    fn insert(&self, category: &Category) -> Result<Category, CategoryError>;
    fn update(&self, category: &Category) -> Result<(), CategoryError>;
    fn update_partial(&self, id: CategoryId, update: CategoryUpdate) -> Result<(), CategoryError>;
    fn delete(&self, id: CategoryId) -> Result<(), CategoryError>;
}

impl From<CategoryError> for MmexError {
    fn from(e: CategoryError) -> Self {
        match e {
            CategoryError::Common(c) => c,
            _ => MmexError::Internal(e.to_string()),
        }
    }
}
