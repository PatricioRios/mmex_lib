use crate::domain::tags::{Tag, TagError, TagId, TagRepository};
use crate::infrastructure::tags_repository::SqlTagRepository;
use rusqlite::Connection;

pub struct TagService<'a> {
    conn: &'a Connection,
}

impl<'a> TagService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_tags(&self) -> Result<Vec<Tag>, TagError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_tag_by_id(&self, id: TagId) -> Result<Option<Tag>, TagError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_tag(&self, name: &str) -> Result<Tag, TagError> {
        if name.trim().is_empty() {
            return Err(TagError::NameRequired);
        }
        let repo = SqlTagRepository::new(self.conn);
        repo.insert(name)
    }

    pub fn update_tag(&self, tag: &Tag) -> Result<(), TagError> {
        if tag.name.trim().is_empty() {
            return Err(TagError::NameRequired);
        }
        let repo = SqlTagRepository::new(self.conn);
        repo.update(tag)
    }

    pub fn delete_tag(&self, id: TagId) -> Result<(), TagError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.delete(id)
    }

    // Funciones de vinculación expuestas

    pub fn get_for_reference(&self, ref_type: &str, ref_id: i64) -> Result<Vec<Tag>, TagError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.find_for_reference(ref_type, ref_id)
    }

    pub fn link_to_reference(
        &self,
        ref_type: &str,
        ref_id: i64,
        tag_id: TagId,
    ) -> Result<(), TagError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.link_to_reference(ref_type, ref_id, tag_id)
    }

    pub fn unlink_from_reference(
        &self,
        ref_type: &str,
        ref_id: i64,
        tag_id: TagId,
    ) -> Result<(), TagError> {
        let repo = SqlTagRepository::new(self.conn);
        repo.unlink_from_reference(ref_type, ref_id, tag_id)
    }
}
