use rusqlite::Row;
use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;

use crate::domain::payees::{Payee, PayeeError, PayeeId, PayeeRepository};
use crate::infrastructure::db_executor::DbExecutor;
use crate::MmexError;

pub struct PayeeMapper;

impl PayeeMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Payee> {
        Ok(Payee {
            id: PayeeId::new(row.get("PAYEEID")?),
            name: row.get("PAYEENAME")?,
            category_id: row.get("CATEGID")?,
            number: row.get("NUMBER")?,
            website: row.get("WEBSITE")?,
            notes: row.get("NOTES")?,
            active: row.get::<_, i32>("ACTIVE")? != 0,
            pattern: row.get("PATTERN")?,
        })
    }
}

pub struct SqlPayeeRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlPayeeRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> PayeeRepository for SqlPayeeRepository<'a, E> {
    fn find_all(&self) -> Result<Vec<Payee>, PayeeError> {
        let (sql, _) = Query::select()
            .columns([
                "PAYEEID",
                "PAYEENAME",
                "CATEGID",
                "NUMBER",
                "WEBSITE",
                "NOTES",
                "ACTIVE",
                "PATTERN",
            ])
            .from(Alias::new("PAYEE_V1"))
            .build(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| PayeeMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: PayeeId) -> Result<Option<Payee>, PayeeError> {
        let (sql, values) = Query::select()
            .columns([
                "PAYEEID",
                "PAYEENAME",
                "CATEGID",
                "NUMBER",
                "WEBSITE",
                "NOTES",
                "ACTIVE",
                "PATTERN",
            ])
            .from(Alias::new("PAYEE_V1"))
            .and_where(Expr::col(Alias::new("PAYEEID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |row| {
                PayeeMapper::map_row(row)
            }) {
            Ok(payee) => Ok(Some(payee)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(PayeeError::Common(e)),
        }
    }

    fn insert(&self, p: &Payee) -> Result<Payee, PayeeError> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("PAYEE_V1"))
            .columns([
                Alias::new("PAYEENAME"),
                Alias::new("CATEGID"),
                Alias::new("NUMBER"),
                Alias::new("WEBSITE"),
                Alias::new("NOTES"),
                Alias::new("ACTIVE"),
                Alias::new("PATTERN"),
            ])
            .values_panic([
                p.name.clone().into(),
                p.category_id.into(),
                p.number.clone().into(),
                p.website.clone().into(),
                p.notes.clone().into(),
                (if p.active { 1 } else { 0 }).into(),
                p.pattern.clone().into(),
            ])
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;

        let mut new_payee = p.clone();
        new_payee.id = PayeeId::new(last_id);
        Ok(new_payee)
    }

    fn update(&self, p: &Payee) -> Result<(), PayeeError> {
        let (sql, values) = Query::update()
            .table(Alias::new("PAYEE_V1"))
            .values([
                (Alias::new("PAYEENAME"), p.name.clone().into()),
                (Alias::new("CATEGID"), p.category_id.into()),
                (Alias::new("NUMBER"), p.number.clone().into()),
                (Alias::new("WEBSITE"), p.website.clone().into()),
                (Alias::new("NOTES"), p.notes.clone().into()),
                (Alias::new("ACTIVE"), (if p.active { 1 } else { 0 }).into()),
                (Alias::new("PATTERN"), p.pattern.clone().into()),
            ])
            .and_where(Expr::col(Alias::new("PAYEEID")).eq(p.id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn delete(&self, id: PayeeId) -> Result<(), PayeeError> {
        let (sql, values) = Query::delete()
            .from_table(Alias::new("PAYEE_V1"))
            .and_where(Expr::col(Alias::new("PAYEEID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }
}
