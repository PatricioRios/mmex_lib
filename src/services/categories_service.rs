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
}
