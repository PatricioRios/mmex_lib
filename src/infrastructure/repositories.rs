use sea_query::{Expr, Query, SqliteQueryBuilder};

use crate::domain::models::Account;
use crate::domain::repositories::AccountRepository;
use crate::domain::types::AccountId;
use crate::error::MmexError;
use crate::infrastructure::db_executor::DbExecutor;
use crate::infrastructure::mapper::AccountMapper;

pub struct SqlAccountRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlAccountRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> AccountRepository for SqlAccountRepository<'a, E> {
    fn find_all(&self) -> Result<Vec<Account>, MmexError> {
        let (sql, _) = Query::select()
            .columns([
                "ACCOUNTID",
                "ACCOUNTNAME",
                "INITIALBAL",
                "ACCOUNTTYPE",
                "ACCOUNTSTATUS",
            ])
            .from_as("ACCOUNTLIST_V1", "a")
            .build(SqliteQueryBuilder);

        let accounts = self.executor.query_map_ext(&sql, [], |row| {
            AccountMapper::map_row(row)
        })?;
            
        Ok(accounts)
    }

    fn find_by_id(&self, id: AccountId) -> Result<Option<Account>, MmexError> {
        let (sql, _) = Query::select()
            .columns([
                "ACCOUNTID",
                "ACCOUNTNAME",
                "INITIALBAL",
                "ACCOUNTTYPE",
                "ACCOUNTSTATUS",
            ])
            .from_as("ACCOUNTLIST_V1", "a")
            .and_where(Expr::col("ACCOUNTID").eq(id.0))
            .build(SqliteQueryBuilder);

        match self.executor.query_row_ext(&sql, [id.0], |row| AccountMapper::map_row(row)) {
            Ok(account) => Ok(Some(account)),
            Err(MmexError::Database(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
