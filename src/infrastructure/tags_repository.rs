use rusqlite::Row;
use sea_query::{Expr, Query, SqliteQueryBuilder, JoinType};

use crate::domain::tags::{Tag, TagId, TagRepository};
use crate::error::MmexError;
use crate::infrastructure::db_executor::DbExecutor;

pub struct TagMapper;

impl TagMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Tag> {
        Ok(Tag {
            id: TagId(row.get("TAGID")?),
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
    fn find_all(&self) -> Result<Vec<Tag>, MmexError> {
        let (sql, _) = Query::select()
            .columns(["TAGID", "TAGNAME"])
            .from_as("TAG_V1", "t")
            .build(SqliteQueryBuilder);

        self.executor.query_map_ext(&sql, [], |row| TagMapper::map_row(row))
    }

    fn find_by_id(&self, id: TagId) -> Result<Option<Tag>, MmexError> {
        let (sql, _) = Query::select()
            .columns(["TAGID", "TAGNAME"])
            .from_as("TAG_V1", "t")
            .and_where(Expr::col("TAGID").eq(id.0))
            .build(SqliteQueryBuilder);

        match self.executor.query_row_ext(&sql, [id.0], |row| TagMapper::map_row(row)) {
            Ok(tag) => Ok(Some(tag)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn insert(&self, name: &str) -> Result<Tag, MmexError> {
        let sql = "INSERT INTO TAG_V1 (TAGNAME, ACTIVE) VALUES (?, 1)";
        self.executor.execute_ext(sql, [name])?;
        let last_id: i64 = self.executor.query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;
        Ok(Tag { id: TagId(last_id), name: name.to_string() })
    }

    fn update(&self, tag: &Tag) -> Result<(), MmexError> {
        let sql = "UPDATE TAG_V1 SET TAGNAME = ? WHERE TAGID = ?";
        self.executor.execute_ext(sql, (&tag.name, tag.id.0))?;
        Ok(())
    }

    fn delete(&self, id: TagId) -> Result<(), MmexError> {
        self.executor.execute_ext("DELETE FROM TAGLINK_V1 WHERE TAGID = ?", [id.0])?;
        self.executor.execute_ext("DELETE FROM TAG_V1 WHERE TAGID = ?", [id.0])?;
        Ok(())
    }

    fn find_for_reference(&self, ref_type: &str, ref_id: i64) -> Result<Vec<Tag>, MmexError> {
        let (sql, _) = Query::select()
            .columns([("t", "TAGID"), ("t", "TAGNAME")])
            .from_as("TAG_V1", "t")
            .join(JoinType::InnerJoin, "TAGLINK_V1", Expr::col(("TAGLINK_V1", "TAGID")).equals(("t", "TAGID")))
            .and_where(Expr::col(("TAGLINK_V1", "REFTYPE")).eq(ref_type))
            .and_where(Expr::col(("TAGLINK_V1", "REFID")).eq(ref_id))
            .build(SqliteQueryBuilder);

        self.executor.query_map_ext(&sql, [ref_type, &ref_id.to_string()], |row| TagMapper::map_row(row))
    }

    fn link_to_reference(&self, ref_type: &str, ref_id: i64, tag_id: TagId) -> Result<(), MmexError> {
        let sql = "INSERT OR IGNORE INTO TAGLINK_V1 (REFTYPE, REFID, TAGID) VALUES (?, ?, ?)";
        self.executor.execute_ext(sql, (ref_type, ref_id, tag_id.0))?;
        Ok(())
    }

    fn unlink_from_reference(&self, ref_type: &str, ref_id: i64, tag_id: TagId) -> Result<(), MmexError> {
        let sql = "DELETE FROM TAGLINK_V1 WHERE REFTYPE = ? AND REFID = ? AND TAGID = ?";
        self.executor.execute_ext(sql, (ref_type, ref_id, tag_id.0))?;
        Ok(())
    }
}
