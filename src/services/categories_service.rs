use rusqlite::Connection;
use crate::domain::categories::{Category, CategoryId, CategoryRepository};
use crate::infrastructure::categories_repository::SqlCategoryRepository;
use crate::error::MmexError;

pub struct CategoryService<'a> {
    conn: &'a Connection,
}

impl<'a> CategoryService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_categories(&self) -> Result<Vec<Category>, MmexError> {
        let repo = SqlCategoryRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_category_by_id(&self, id: CategoryId) -> Result<Option<Category>, MmexError> {
        let repo = SqlCategoryRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn get_subcategories(&self, parent_id: CategoryId) -> Result<Vec<Category>, MmexError> {
        let repo = SqlCategoryRepository::new(self.conn);
        repo.find_subcategories(parent_id)
    }

    pub fn create_category(&self, name: &str, parent_id: Option<CategoryId>) -> Result<Category, MmexError> {
        if name.trim().is_empty() {
            return Err(MmexError::Validation("Category name is required".into()));
        }
        let repo = SqlCategoryRepository::new(self.conn);
        let new_cat = Category {
            id: CategoryId(0),
            name: name.to_string(),
            active: true,
            parent_id,
        };
        repo.insert(&new_cat)
    }

    pub fn update_category(&self, category: &Category) -> Result<(), MmexError> {
        let repo = SqlCategoryRepository::new(self.conn);
        repo.update(category)
    }

    pub fn delete_category(&self, id: CategoryId) -> Result<(), MmexError> {
        let repo = SqlCategoryRepository::new(self.conn);
        repo.delete(id)
    }
}
