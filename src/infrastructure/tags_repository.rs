use rusqlite::Row;
use sea_query::{Alias, Expr, JoinType, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;

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
        let (sql, values) = Query::select()
            .columns(["TAGID", "TAGNAME"])
            .from(Alias::new("TAG_V1"))
            .and_where(Expr::col(Alias::new("ACTIVE")).eq(1))
            .build_rusqlite(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, &values.as_params()[..], |row| TagMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: TagId) -> Result<Option<Tag>, TagError> {
        let (sql, values) = Query::select()
            .columns(["TAGID", "TAGNAME"])
            .from(Alias::new("TAG_V1"))
            .and_where(Expr::col(Alias::new("TAGID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |row| TagMapper::map_row(row))
        {
            Ok(tag) => Ok(Some(tag)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(TagError::Common(e)),
        }
    }

    fn insert(&self, name: &str) -> Result<Tag, TagError> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("TAG_V1"))
            .columns([Alias::new("TAGNAME"), Alias::new("ACTIVE")])
            .values_panic([name.into(), 1.into()])
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;

        Ok(Tag {
            id: TagId::new(last_id),
            name: name.to_string(),
        })
    }

    fn update(&self, tag: &Tag) -> Result<(), TagError> {
        let (sql, values) = Query::update()
            .table(Alias::new("TAG_V1"))
            .values([(Alias::new("TAGNAME"), tag.name.clone().into())])
            .and_where(Expr::col(Alias::new("TAGID")).eq(tag.id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn delete(&self, id: TagId) -> Result<(), TagError> {
        let (sql_link, values_link) = Query::delete()
            .from_table(Alias::new("TAGLINK_V1"))
            .and_where(Expr::col(Alias::new("TAGID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);
        self.executor
            .execute_ext(&sql_link, &values_link.as_params()[..])?;

        let (sql, values) = Query::delete()
            .from_table(Alias::new("TAG_V1"))
            .and_where(Expr::col(Alias::new("TAGID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn find_for_reference(&self, ref_type: &str, ref_id: i64) -> Result<Vec<Tag>, TagError> {
        let (sql, values) = Query::select()
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
            .build_rusqlite(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, &values.as_params()[..], |row| TagMapper::map_row(row))?)
    }

    fn link_to_reference(
        &self,
        ref_type: &str,
        ref_id: i64,
        tag_id: TagId,
    ) -> Result<(), TagError> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("TAGLINK_V1"))
            .columns([
                Alias::new("REFTYPE"),
                Alias::new("REFID"),
                Alias::new("TAGID"),
            ])
            .values_panic([ref_type.into(), ref_id.into(), tag_id.v1.into()])
            .build_rusqlite(SqliteQueryBuilder);

        let sql = sql.replace("INSERT INTO", "INSERT OR IGNORE INTO");

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn unlink_from_reference(
        &self,
        ref_type: &str,
        ref_id: i64,
        tag_id: TagId,
    ) -> Result<(), TagError> {
        let (sql, values) = Query::delete()
            .from_table(Alias::new("TAGLINK_V1"))
            .and_where(Expr::col(Alias::new("REFTYPE")).eq(ref_type))
            .and_where(Expr::col(Alias::new("REFID")).eq(ref_id))
            .and_where(Expr::col(Alias::new("TAGID")).eq(tag_id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }
}
