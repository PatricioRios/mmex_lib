use rusqlite::Row;
use sea_query::{Alias, Expr, JoinType, Query, SqliteQueryBuilder};

use crate::domain::tags::{Tag, TagError, TagId, TagRepository};
use crate::infrastructure::db_executor::DbExecutor;
use crate::MmexError;

pub struct TagMapper;

impl TagMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Tag> {
        Ok(Tag {
            id: TagId::new(row.get("TAGID")?),
            name: row.get("TAGNAME")?,
        })
    }
}

pub struct SqlTagRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlTagRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> TagRepository for SqlTagRepository<'a, E> {
    fn find_all(&self) -> Result<Vec<Tag>, TagError> {
        let (sql, _) = Query::select()
            .columns(["TAGID", "TAGNAME"])
            .from(Alias::new("TAG_V1"))
            .and_where(Expr::col(Alias::new("ACTIVE")).eq(1))
            .build(SqliteQueryBuilder);
        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| TagMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: TagId) -> Result<Option<Tag>, TagError> {
        let (sql, _) = Query::select()
            .columns(["TAGID", "TAGNAME"])
            .from(Alias::new("TAG_V1"))
            .and_where(Expr::col(Alias::new("TAGID")).eq(id.v1))
            .build(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, [id.v1], |row| TagMapper::map_row(row))
        {
            Ok(tag) => Ok(Some(tag)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(TagError::Common(e)),
        }
    }

    fn insert(&self, name: &str) -> Result<Tag, TagError> {
        let sql = "INSERT INTO TAG_V1 (TAGNAME, ACTIVE) VALUES (?, 1)";
        self.executor.execute_ext(sql, [name])?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;

        Ok(Tag {
            id: TagId::new(last_id),
            name: name.to_string(),
        })
    }

    fn update(&self, tag: &Tag) -> Result<(), TagError> {
        let sql = "UPDATE TAG_V1 SET TAGNAME = ? WHERE TAGID = ?";
        self.executor.execute_ext(sql, (&tag.name, tag.id.v1))?;
        Ok(())
    }

    fn delete(&self, id: TagId) -> Result<(), TagError> {
        self.executor
            .execute_ext("DELETE FROM TAGLINK_V1 WHERE TAGID = ?", [id.v1])?;
        self.executor
            .execute_ext("DELETE FROM TAG_V1 WHERE TAGID = ?", [id.v1])?;
        Ok(())
    }

    fn find_for_reference(&self, ref_type: &str, ref_id: i64) -> Result<Vec<Tag>, TagError> {
        let (sql, _) = Query::select()
            .columns([
                (Alias::new("t"), Alias::new("TAGID")),
                (Alias::new("t"), Alias::new("TAGNAME")),
            ])
            .from_as(Alias::new("TAG_V1"), Alias::new("t"))
            .join(
                JoinType::InnerJoin,
                Alias::new("TAGLINK_V1"),
                Expr::col((Alias::new("TAGLINK_V1"), Alias::new("TAGID")))
                    .equals((Alias::new("t"), Alias::new("TAGID"))),
            )
            .and_where(Expr::col((Alias::new("TAGLINK_V1"), Alias::new("REFTYPE"))).eq(ref_type))
            .and_where(Expr::col((Alias::new("TAGLINK_V1"), Alias::new("REFID"))).eq(ref_id))
            .build(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, (ref_type, ref_id), |row| TagMapper::map_row(row))?)
    }

    fn link_to_reference(
        &self,
        ref_type: &str,
        ref_id: i64,
        tag_id: TagId,
    ) -> Result<(), TagError> {
        let sql = "INSERT OR IGNORE INTO TAGLINK_V1 (REFTYPE, REFID, TAGID) VALUES (?, ?, ?)";
        self.executor
            .execute_ext(sql, (ref_type, ref_id, tag_id.v1))?;
        Ok(())
    }

    fn unlink_from_reference(
        &self,
        ref_type: &str,
        ref_id: i64,
        tag_id: TagId,
    ) -> Result<(), TagError> {
        let sql = "DELETE FROM TAGLINK_V1 WHERE REFTYPE = ? AND REFID = ? AND TAGID = ?";
        self.executor
            .execute_ext(sql, (ref_type, ref_id, tag_id.v1))?;
        Ok(())
    }
}
