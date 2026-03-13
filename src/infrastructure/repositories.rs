use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;

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
        let (sql, values) = Query::select()
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
            .from(Alias::new("ACCOUNTLIST_V1"))
            .build_rusqlite(SqliteQueryBuilder);
        Ok(self
            .executor
            .query_map_ext(&sql, &values.as_params()[..], |row| {
                AccountMapper::map_row(row)
            })?)
    }

    fn find_by_id(&self, id: AccountId) -> Result<Option<Account>, AccountError> {
        let (sql, values) = Query::select()
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
            .from(Alias::new("ACCOUNTLIST_V1"))
            .and_where(Expr::col(Alias::new("ACCOUNTID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |row| {
                AccountMapper::map_row(row)
            }) {
            Ok(account) => Ok(Some(account)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(AccountError::Common(e)),
        }
    }

    fn insert(&self, a: &Account) -> Result<Account, AccountError> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("ACCOUNTLIST_V1"))
            .columns([
                Alias::new("ACCOUNTNAME"),
                Alias::new("ACCOUNTTYPE"),
                Alias::new("ACCOUNTNUM"),
                Alias::new("STATUS"),
                Alias::new("NOTES"),
                Alias::new("INITIALBAL"),
                Alias::new("CURRENCYID"),
                Alias::new("FAVORITEACCT"),
            ])
            .values_panic([
                a.name.clone().into(),
                a.account_type.to_string().into(),
                a.account_num.clone().into(),
                a.status.to_string().into(),
                a.notes.clone().into(),
                a.initial_balance.v1.clone().into(),
                a.currency_id.v1.into(),
                (if a.favorite { "1" } else { "0" }).into(),
            ])
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;

        let mut new_acc = a.clone();
        new_acc.id = AccountId::new(last_id);
        Ok(new_acc)
    }

    fn update(&self, a: &Account) -> Result<(), AccountError> {
        let (sql, values) = Query::update()
            .table(Alias::new("ACCOUNTLIST_V1"))
            .values([
                (Alias::new("ACCOUNTNAME"), a.name.clone().into()),
                (Alias::new("ACCOUNTTYPE"), a.account_type.to_string().into()),
                (Alias::new("ACCOUNTNUM"), a.account_num.clone().into()),
                (Alias::new("STATUS"), a.status.to_string().into()),
                (Alias::new("NOTES"), a.notes.clone().into()),
                (
                    Alias::new("INITIALBAL"),
                    a.initial_balance.v1.clone().into(),
                ),
                (Alias::new("CURRENCYID"), a.currency_id.v1.into()),
                (
                    Alias::new("FAVORITEACCT"),
                    (if a.favorite { "1" } else { "0" }).into(),
                ),
            ])
            .and_where(Expr::col(Alias::new("ACCOUNTID")).eq(a.id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn delete(&self, id: AccountId) -> Result<(), AccountError> {
        let (sql, values) = Query::delete()
            .from_table(Alias::new("ACCOUNTLIST_V1"))
            .and_where(Expr::col(Alias::new("ACCOUNTID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
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
        let (sql, values) = Query::select()
            .column(Alias::new("INFOVALUE"))
            .from(Alias::new("INFOTABLE_V1"))
            .and_where(Expr::col(Alias::new("INFONAME")).eq(name))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |r| r.get(0))
        {
            Ok(val) => Ok(Some(val)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(SupportError::Common(e)),
        }
    }

    fn get_setting(&self, name: &str) -> Result<Option<String>, SupportError> {
        let (sql, values) = Query::select()
            .column(Alias::new("SETTINGVALUE"))
            .from(Alias::new("SETTING_V1"))
            .and_where(Expr::col(Alias::new("SETTINGNAME")).eq(name))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |r| r.get(0))
        {
            Ok(val) => Ok(Some(val)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(SupportError::Common(e)),
        }
    }

    fn set_setting(&self, name: &str, value: &str) -> Result<(), SupportError> {
        let sql = "INSERT OR REPLACE INTO SETTING_V1 (SETTINGNAME, SETTINGVALUE) VALUES (?, ?)";
        self.executor.execute_ext(sql, [name, value])?;
        Ok(())
    }
}
