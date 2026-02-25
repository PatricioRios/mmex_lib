use rusqlite::Row;
use sea_query::{Expr, Query, SqliteQueryBuilder};

use crate::domain::categories::{Category, CategoryId, CategoryRepository};
use crate::error::MmexError;
use crate::infrastructure::db_executor::DbExecutor;

pub struct CategoryMapper;

impl CategoryMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Category> {
        let parent_id_val: i32 = row.get("PARENTID")?;
        let parent_id = if parent_id_val == -1 {
            None
        } else {
            Some(CategoryId(parent_id_val))
        };

        Ok(Category {
            id: CategoryId(row.get("CATEGID")?),
            name: row.get("CATEGNAME")?,
            active: row.get::<_, i32>("ACTIVE")? != 0,
            parent_id,
        })
    }
}

pub struct SqlCategoryRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlCategoryRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> CategoryRepository for SqlCategoryRepository<'a, E> {
    fn find_all(&self) -> Result<Vec<Category>, MmexError> {
        let (sql, _) = Query::select()
            .columns(["CATEGID", "CATEGNAME", "ACTIVE", "PARENTID"])
            .from_as("CATEGORY_V1", "c")
            .build(SqliteQueryBuilder);

        self.executor.query_map_ext(&sql, [], |row| CategoryMapper::map_row(row))
    }

    fn find_by_id(&self, id: CategoryId) -> Result<Option<Category>, MmexError> {
        let (sql, _) = Query::select()
            .columns(["CATEGID", "CATEGNAME", "ACTIVE", "PARENTID"])
            .from_as("CATEGORY_V1", "c")
            .and_where(Expr::col("CATEGID").eq(id.0))
            .build(SqliteQueryBuilder);

        match self.executor.query_row_ext(&sql, [id.0], |row| CategoryMapper::map_row(row)) {
            Ok(cat) => Ok(Some(cat)),
            Err(MmexError::Database(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn find_subcategories(&self, parent_id: CategoryId) -> Result<Vec<Category>, MmexError> {
        let (sql, _) = Query::select()
            .columns(["CATEGID", "CATEGNAME", "ACTIVE", "PARENTID"])
            .from_as("CATEGORY_V1", "c")
            .and_where(Expr::col("PARENTID").eq(parent_id.0))
            .build(SqliteQueryBuilder);

        self.executor.query_map_ext(&sql, [parent_id.0], |row| CategoryMapper::map_row(row))
    }
}
