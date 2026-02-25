use rusqlite::Row;
use sea_query::{Expr, Query, SqliteQueryBuilder};

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
            .from_as("TAGS_V1", "t")
            .build(SqliteQueryBuilder);

        self.executor.query_map_ext(&sql, [], |row| TagMapper::map_row(row))
    }

    fn find_by_id(&self, id: TagId) -> Result<Option<Tag>, MmexError> {
        let (sql, _) = Query::select()
            .columns(["TAGID", "TAGNAME"])
            .from_as("TAGS_V1", "t")
            .and_where(Expr::col("TAGID").eq(id.0))
            .build(SqliteQueryBuilder);

        match self.executor.query_row_ext(&sql, [id.0], |row| TagMapper::map_row(row)) {
            Ok(tag) => Ok(Some(tag)),
            Err(MmexError::Database(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn insert(&self, name: &str) -> Result<Tag, MmexError> {
        let sql = "INSERT INTO TAGS_V1 (TAGNAME) VALUES (?)";
        self.executor.execute_ext(sql, [name])?;
        
        // En SQLite, obtener el último ID insertado
        let last_id: i32 = self.executor.query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;
        
        Ok(Tag {
            id: TagId(last_id),
            name: name.to_string(),
        })
    }
}
