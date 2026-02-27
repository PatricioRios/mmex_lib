use rusqlite::Row;
use sea_query::{Expr, Query, SqliteQueryBuilder};

use crate::domain::payees::{Payee, PayeeId, PayeeRepository};
use crate::error::MmexError;
use crate::infrastructure::db_executor::DbExecutor;

pub struct PayeeMapper;

impl PayeeMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Payee> {
        Ok(Payee {
            id: PayeeId(row.get("PAYEEID")?),
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
    fn find_all(&self) -> Result<Vec<Payee>, MmexError> {
        let (sql, _) = Query::select()
            .columns([
                "PAYEEID", "PAYEENAME", "CATEGID", "NUMBER", 
                "WEBSITE", "NOTES", "ACTIVE", "PATTERN"
            ])
            .from_as("PAYEE_V1", "p")
            .build(SqliteQueryBuilder);

        self.executor.query_map_ext(&sql, [], |row| PayeeMapper::map_row(row))
    }

    fn find_by_id(&self, id: PayeeId) -> Result<Option<Payee>, MmexError> {
        let (sql, _) = Query::select()
            .columns([
                "PAYEEID", "PAYEENAME", "CATEGID", "NUMBER", 
                "WEBSITE", "NOTES", "ACTIVE", "PATTERN"
            ])
            .from_as("PAYEE_V1", "p")
            .and_where(Expr::col("PAYEEID").eq(id.0))
            .build(SqliteQueryBuilder);

        match self.executor.query_row_ext(&sql, [id.0], |row| PayeeMapper::map_row(row)) {
            Ok(payee) => Ok(Some(payee)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn insert(&self, p: &Payee) -> Result<Payee, MmexError> {
        let sql = "INSERT INTO PAYEE_V1 (PAYEENAME, CATEGID, NUMBER, WEBSITE, NOTES, ACTIVE, PATTERN) 
                   VALUES (?, ?, ?, ?, ?, ?, ?)";
        self.executor.execute_ext(sql, (&p.name, p.category_id, &p.number, &p.website, &p.notes, if p.active { 1 } else { 0 }, &p.pattern))?;
        let last_id: i64 = self.executor.query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;
        let mut new_payee = p.clone();
        new_payee.id = PayeeId(last_id);
        Ok(new_payee)
    }

    fn update(&self, p: &Payee) -> Result<(), MmexError> {
        let sql = "UPDATE PAYEE_V1 SET 
                   PAYEENAME = ?, CATEGID = ?, NUMBER = ?, WEBSITE = ?, NOTES = ?, ACTIVE = ?, PATTERN = ?
                   WHERE PAYEEID = ?";
        self.executor.execute_ext(sql, (&p.name, p.category_id, &p.number, &p.website, &p.notes, if p.active { 1 } else { 0 }, &p.pattern, p.id.0))?;
        Ok(())
    }

    fn delete(&self, id: PayeeId) -> Result<(), MmexError> {
        self.executor.execute_ext("DELETE FROM PAYEE_V1 WHERE PAYEEID = ?", [id.0])?;
        Ok(())
    }
}
