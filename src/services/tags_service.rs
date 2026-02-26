use rusqlite::Connection;
use crate::domain::tags::{Tag, TagId, TagRepository};
use crate::infrastructure::tags_repository::SqlTagRepository;
use crate::error::MmexError;

pub struct TagService<'a> {
    conn: &'a Connection,
}

impl<'a> TagService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_tags(&self) -> Result<Vec<Tag>, MmexError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_tag_by_id(&self, id: TagId) -> Result<Option<Tag>, MmexError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_tag(&self, name: &str) -> Result<Tag, MmexError> {
        if name.trim().is_empty() {
            return Err(MmexError::Validation("Tag name cannot be empty".into()));
        }
        let repo = SqlTagRepository::new(self.conn);
        repo.insert(name)
    }

    pub fn update_tag(&self, tag: &Tag) -> Result<(), MmexError> {
        if tag.name.trim().is_empty() {
            return Err(MmexError::Validation("Tag name cannot be empty".into()));
        }
        let repo = SqlTagRepository::new(self.conn);
        repo.update(tag)
    }

    pub fn delete_tag(&self, id: TagId) -> Result<(), MmexError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.delete(id)
    }
}
