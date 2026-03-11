use sea_query::{Expr, Query, SqliteQueryBuilder};

use crate::domain::accounts::{Account, AccountError, AccountId, AccountRepository};
use crate::domain::models::{SupportError, SupportRepository};
use crate::infrastructure::db_executor::DbExecutor;
use crate::infrastructure::mapper::AccountMapper;
use crate::MmexError;

pub struct SqlAccountRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlAccountRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> AccountRepository for SqlAccountRepository<'a, E> {
    fn find_all(&self) -> Result<Vec<Account>, AccountError> {
        let (sql, _) = Query::select()
            .columns([
                "ACCOUNTID",
                "ACCOUNTNAME",
                "ACCOUNTTYPE",
                "ACCOUNTNUM",
                "STATUS",
                "NOTES",
                "INITIALBAL",
                "CURRENCYID",
                "FAVORITEACCT",
            ])
            .from_as("ACCOUNTLIST_V1", "a")
            .build(SqliteQueryBuilder);
        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| AccountMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: AccountId) -> Result<Option<Account>, AccountError> {
        let (sql, _) = Query::select()
            .columns([
                "ACCOUNTID",
                "ACCOUNTNAME",
                "ACCOUNTTYPE",
                "ACCOUNTNUM",
                "STATUS",
                "NOTES",
                "INITIALBAL",
                "CURRENCYID",
                "FAVORITEACCT",
            ])
            .from_as("ACCOUNTLIST_V1", "a")
            .and_where(Expr::col("ACCOUNTID").eq(id.0))
            .build(SqliteQueryBuilder);
        match self
            .executor
            .query_row_ext(&sql, [id.0], |row| AccountMapper::map_row(row))
        {
            Ok(account) => Ok(Some(account)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(AccountError::Common(e)),
        }
    }

    fn insert(&self, a: &Account) -> Result<Account, AccountError> {
        let sql = "INSERT INTO ACCOUNTLIST_V1 (ACCOUNTNAME, ACCOUNTTYPE, ACCOUNTNUM, STATUS, NOTES, INITIALBAL, CURRENCYID, FAVORITEACCT) 
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
        self.executor.execute_ext(
            sql,
            (
                &a.name,
                a.account_type.to_string(),
                &a.account_num,
                a.status.to_string(),
                &a.notes,
                a.initial_balance.0.to_string(),
                a.currency_id.0,
                if a.favorite { "1" } else { "0" },
            ),
        )?;
        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;
        let mut new_acc = a.clone();
        new_acc.id = AccountId(last_id);
        Ok(new_acc)
    }

    fn update(&self, a: &Account) -> Result<(), AccountError> {
        let sql = "UPDATE ACCOUNTLIST_V1 SET 
                   ACCOUNTNAME = ?, ACCOUNTTYPE = ?, ACCOUNTNUM = ?, STATUS = ?, NOTES = ?, INITIALBAL = ?, CURRENCYID = ?, FAVORITEACCT = ?
                   WHERE ACCOUNTID = ?";
        self.executor.execute_ext(
            sql,
            (
                &a.name,
                a.account_type.to_string(),
                &a.account_num,
                a.status.to_string(),
                &a.notes,
                a.initial_balance.0.to_string(),
                a.currency_id.0,
                if a.favorite { "1" } else { "0" },
                a.id.0,
            ),
        )?;
        Ok(())
    }

    fn delete(&self, id: AccountId) -> Result<(), AccountError> {
        self.executor
            .execute_ext("DELETE FROM ACCOUNTLIST_V1 WHERE ACCOUNTID = ?", [id.0])?;
        Ok(())
    }
}

pub struct SqlSupportRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlSupportRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> SupportRepository for SqlSupportRepository<'a, E> {
    fn get_metadata(&self, name: &str) -> Result<Option<String>, SupportError> {
        let sql = "SELECT INFOVALUE FROM INFOTABLE_V1 WHERE INFONAME = ?";
        match self.executor.query_row_ext(sql, [name], |r| r.get(0)) {
            Ok(val) => Ok(Some(val)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(SupportError::Common(e)),
        }
    }

    fn get_setting(&self, name: &str) -> Result<Option<String>, SupportError> {
        let sql = "SELECT SETTINGVALUE FROM SETTING_V1 WHERE SETTINGNAME = ?";
        match self.executor.query_row_ext(sql, [name], |r| r.get(0)) {
            Ok(val) => Ok(Some(val)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(SupportError::Common(e)),
        }
    }

    fn set_setting(&self, name: &str, value: &str) -> Result<(), SupportError> {
        let sql = "INSERT OR REPLACE INTO SETTING_V1 (SETTINGNAME, SETTINGVALUE) VALUES (?, ?)";
        self.executor.execute_ext(sql, [name, value])?;
        Ok(())
    }
}
