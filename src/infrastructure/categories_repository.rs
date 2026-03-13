use rusqlite::Row;
use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;

use crate::domain::categories::{Category, CategoryError, CategoryId, CategoryRepository};
use crate::infrastructure::db_executor::DbExecutor;
use crate::MmexError;

pub struct CategoryMapper;

impl CategoryMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Category> {
        let parent_id_val: i64 = row.get("PARENTID")?;
        let parent_id = if parent_id_val == -1 {
            None
        } else {
            Some(CategoryId::new(parent_id_val))
        };

        Ok(Category {
            id: CategoryId::new(row.get("CATEGID")?),
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
    fn find_all(&self) -> Result<Vec<Category>, CategoryError> {
        let (sql, _) = Query::select()
            .columns(["CATEGID", "CATEGNAME", "ACTIVE", "PARENTID"])
            .from(Alias::new("CATEGORY_V1"))
            .build(SqliteQueryBuilder);
        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| CategoryMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: CategoryId) -> Result<Option<Category>, CategoryError> {
        let (sql, values) = Query::select()
            .columns(["CATEGID", "CATEGNAME", "ACTIVE", "PARENTID"])
            .from(Alias::new("CATEGORY_V1"))
            .and_where(Expr::col(Alias::new("CATEGID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |row| {
                CategoryMapper::map_row(row)
            }) {
            Ok(cat) => Ok(Some(cat)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(CategoryError::Common(e)),
        }
    }

    fn find_subcategories(&self, parent_id: CategoryId) -> Result<Vec<Category>, CategoryError> {
        let (sql, values) = Query::select()
            .columns(["CATEGID", "CATEGNAME", "ACTIVE", "PARENTID"])
            .from(Alias::new("CATEGORY_V1"))
            .and_where(Expr::col(Alias::new("PARENTID")).eq(parent_id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, &values.as_params()[..], |row| {
                CategoryMapper::map_row(row)
            })?)
    }

    fn insert(&self, c: &Category) -> Result<Category, CategoryError> {
        let parent_id = c.parent_id.map(|id| id.v1).unwrap_or(-1);

        let (sql, values) = Query::insert()
            .into_table(Alias::new("CATEGORY_V1"))
            .columns([
                Alias::new("CATEGNAME"),
                Alias::new("ACTIVE"),
                Alias::new("PARENTID"),
            ])
            .values_panic([
                c.name.clone().into(),
                (if c.active { 1 } else { 0 }).into(),
                parent_id.into(),
            ])
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;

        let mut new_cat = c.clone();
        new_cat.id = CategoryId::new(last_id);
        Ok(new_cat)
    }

    fn update(&self, c: &Category) -> Result<(), CategoryError> {
        let parent_id = c.parent_id.map(|id| id.v1).unwrap_or(-1);

        let (sql, values) = Query::update()
            .table(Alias::new("CATEGORY_V1"))
            .values([
                (Alias::new("CATEGNAME"), c.name.clone().into()),
                (Alias::new("ACTIVE"), (if c.active { 1 } else { 0 }).into()),
                (Alias::new("PARENTID"), parent_id.into()),
            ])
            .and_where(Expr::col(Alias::new("CATEGID")).eq(c.id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn delete(&self, id: CategoryId) -> Result<(), CategoryError> {
        let (sql, values) = Query::delete()
            .from_table(Alias::new("CATEGORY_V1"))
            .and_where(Expr::col(Alias::new("CATEGID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }
}
