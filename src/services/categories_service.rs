use crate::domain::categories::{Category, CategoryError, CategoryId, CategoryRepository};
use crate::infrastructure::categories_repository::SqlCategoryRepository;
use rusqlite::Connection;

pub struct CategoryService<'a> {
    conn: &'a Connection,
}

impl<'a> CategoryService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_categories(&self) -> Result<Vec<Category>, CategoryError> {
        let repo = SqlCategoryRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_category_by_id(&self, id: CategoryId) -> Result<Option<Category>, CategoryError> {
        let repo = SqlCategoryRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn get_subcategories(&self, parent_id: CategoryId) -> Result<Vec<Category>, CategoryError> {
        let repo = SqlCategoryRepository::new(self.conn);
        repo.find_subcategories(parent_id)
    }

    pub fn create_category(
        &self,
        name: &str,
        parent_id: Option<CategoryId>,
    ) -> Result<Category, CategoryError> {
        if name.trim().is_empty() {
            return Err(CategoryError::NameRequired);
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

    pub fn update_category(&self, category: &Category) -> Result<(), CategoryError> {
        let repo = SqlCategoryRepository::new(self.conn);
        repo.update(category)
    }

    pub fn delete_category(&self, id: CategoryId) -> Result<(), CategoryError> {
        let repo = SqlCategoryRepository::new(self.conn);
        repo.delete(id)
    }
}
