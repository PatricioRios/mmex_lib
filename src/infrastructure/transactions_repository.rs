use chrono::NaiveDate;
use rusqlite::Row;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_query::{Alias, Expr, Query, SimpleExpr, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use std::str::FromStr;

use crate::domain::payees::PayeeId;
use crate::domain::transactions::{
    Transaction, TransactionCode, TransactionError, TransactionId, TransactionRepository,
    TransactionStatus,
};
use crate::domain::types::{AccountId, CategoryId, MmexDate, Money};
use crate::infrastructure::db_executor::DbExecutor;
use crate::MmexError;

pub struct TransactionMapper;

impl TransactionMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Transaction> {
        let amount_val = if let Ok(val) = row.get::<_, f64>("TRANSAMOUNT") {
            Decimal::from_f64(val).unwrap_or(Decimal::ZERO)
        } else if let Ok(s) = row.get::<_, String>("TRANSAMOUNT") {
            Decimal::from_str(&s).unwrap_or(Decimal::ZERO)
        } else {
            Decimal::ZERO
        };

        let to_amount_val = if let Ok(val) = row.get::<_, f64>("TOTRANSAMOUNT") {
            Some(Money::from(Decimal::from_f64(val).unwrap_or(Decimal::ZERO)))
        } else if let Ok(s) = row.get::<_, String>("TOTRANSAMOUNT") {
            Some(Money::from(Decimal::from_str(&s).unwrap_or(Decimal::ZERO)))
        } else {
            None
        };

        let date_str: Option<String> = row.get("TRANSDATE")?;
        let date = date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

        Ok(Transaction {
            id: TransactionId::new(row.get("TRANSID")?),
            account_id: AccountId::new(row.get("ACCOUNTID")?),
            to_account_id: row
                .get::<_, Option<i64>>("TOACCOUNTID")?
                .map(AccountId::new),
            payee_id: PayeeId::new(row.get("PAYEEID")?),
            trans_code: TransactionCode::from(row.get::<_, String>("TRANSCODE")?),
            amount: Money::from(amount_val),
            status: TransactionStatus::from(row.get::<_, String>("STATUS")?),
            transaction_number: row.get("TRANSACTIONNUMBER")?,
            notes: row.get("NOTES")?,
            category_id: row.get::<_, Option<i64>>("CATEGID")?.map(CategoryId::new),
            date: date.map(MmexDate::from),
            to_amount: to_amount_val,
        })
    }
}

pub struct SqlTransactionRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlTransactionRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> TransactionRepository for SqlTransactionRepository<'a, E> {
    fn find_all(&self) -> Result<Vec<Transaction>, TransactionError> {
        let (sql, _) = Query::select()
            .columns([
                "TRANSID",
                "ACCOUNTID",
                "TOACCOUNTID",
                "PAYEEID",
                "TRANSCODE",
                "TRANSAMOUNT",
                "STATUS",
                "TRANSACTIONNUMBER",
                "NOTES",
                "CATEGID",
                "TRANSDATE",
                "TOTRANSAMOUNT",
            ])
            .from(Alias::new("CHECKINGACCOUNT_V1"))
            .build(SqliteQueryBuilder);
        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| TransactionMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: TransactionId) -> Result<Option<Transaction>, TransactionError> {
        let (sql, values) = Query::select()
            .columns([
                "TRANSID",
                "ACCOUNTID",
                "TOACCOUNTID",
                "PAYEEID",
                "TRANSCODE",
                "TRANSAMOUNT",
                "STATUS",
                "TRANSACTIONNUMBER",
                "NOTES",
                "CATEGID",
                "TRANSDATE",
                "TOTRANSAMOUNT",
            ])
            .from(Alias::new("CHECKINGACCOUNT_V1"))
            .and_where(Expr::col(Alias::new("TRANSID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |row| {
                TransactionMapper::map_row(row)
            }) {
            Ok(tx) => Ok(Some(tx)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(TransactionError::Common(e)),
        }
    }

    fn find_for_account(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<Transaction>, TransactionError> {
        let (sql, values) = Query::select()
            .columns([
                "TRANSID",
                "ACCOUNTID",
                "TOACCOUNTID",
                "PAYEEID",
                "TRANSCODE",
                "TRANSAMOUNT",
                "STATUS",
                "TRANSACTIONNUMBER",
                "NOTES",
                "CATEGID",
                "TRANSDATE",
                "TOTRANSAMOUNT",
            ])
            .from(Alias::new("CHECKINGACCOUNT_V1"))
            .and_where(
                Expr::col(Alias::new("ACCOUNTID"))
                    .eq(account_id.v1)
                    .or(Expr::col(Alias::new("TOACCOUNTID")).eq(account_id.v1)),
            )
            .build_rusqlite(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, &values.as_params()[..], |row| {
                TransactionMapper::map_row(row)
            })?)
    }

    fn insert(&self, tx: &Transaction) -> Result<Transaction, TransactionError> {
        let date_str = tx.date.as_ref().map(|d| d.v1.clone());

        let (sql, values) = Query::insert()
            .into_table(Alias::new("CHECKINGACCOUNT_V1"))
            .columns([
                Alias::new("ACCOUNTID"),
                Alias::new("TOACCOUNTID"),
                Alias::new("PAYEEID"),
                Alias::new("TRANSCODE"),
                Alias::new("TRANSAMOUNT"),
                Alias::new("STATUS"),
                Alias::new("TRANSACTIONNUMBER"),
                Alias::new("NOTES"),
                Alias::new("CATEGID"),
                Alias::new("TRANSDATE"),
                Alias::new("TOTRANSAMOUNT"),
            ])
            .values_panic([
                tx.account_id.v1.into(),
                tx.to_account_id.map(|id| id.v1).into(),
                tx.payee_id.v1.into(),
                tx.trans_code.to_string().into(),
                tx.amount.v1.clone().into(),
                tx.status.to_string().into(),
                tx.transaction_number.clone().into(),
                tx.notes.clone().into(),
                tx.category_id.map(|id| id.v1).into(),
                date_str.into(),
                tx.to_amount.as_ref().map(|m| m.v1.clone()).into(),
            ])
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;

        let mut new_tx = tx.clone();
        new_tx.id = TransactionId::new(last_id);
        Ok(new_tx)
    }

    fn update(&self, tx: &Transaction) -> Result<(), TransactionError> {
        let date_str = tx.date.as_ref().map(|d| d.v1.clone());

        let (sql, values) = Query::update()
            .table(Alias::new("CHECKINGACCOUNT_V1"))
            .values([
                (Alias::new("ACCOUNTID"), tx.account_id.v1.into()),
                (
                    Alias::new("TOACCOUNTID"),
                    tx.to_account_id.map(|id| id.v1).into(),
                ),
                (Alias::new("PAYEEID"), tx.payee_id.v1.into()),
                (Alias::new("TRANSCODE"), tx.trans_code.to_string().into()),
                (Alias::new("TRANSAMOUNT"), tx.amount.v1.clone().into()),
                (Alias::new("STATUS"), tx.status.to_string().into()),
                (
                    Alias::new("TRANSACTIONNUMBER"),
                    tx.transaction_number.clone().into(),
                ),
                (Alias::new("NOTES"), tx.notes.clone().into()),
                (Alias::new("CATEGID"), tx.category_id.map(|id| id.v1).into()),
                (Alias::new("TRANSDATE"), date_str.into()),
                (
                    Alias::new("TOTRANSAMOUNT"),
                    tx.to_amount.as_ref().map(|m| m.v1.clone()).into(),
                ),
            ])
            .and_where(Expr::col(Alias::new("TRANSID")).eq(tx.id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn update_partial(
        &self,
        id: TransactionId,
        update: crate::domain::transactions::TransactionUpdate,
    ) -> Result<(), TransactionError> {
        let mut query = Query::update();
        query.table(Alias::new("CHECKINGACCOUNT_V1"));

        let mut has_values = false;

        if let Some(acc_id) = update.account_id {
            query.value(Alias::new("ACCOUNTID"), SimpleExpr::from(acc_id.v1));
            has_values = true;
        }
        if let Some(to_acc_id) = update.to_account_id {
            query.value(Alias::new("TOACCOUNTID"), SimpleExpr::from(to_acc_id.v1));
            has_values = true;
        }
        if let Some(payee_id) = update.payee_id {
            query.value(Alias::new("PAYEEID"), SimpleExpr::from(payee_id.v1));
            has_values = true;
        }
        if let Some(tc) = update.trans_code {
            query.value(Alias::new("TRANSCODE"), SimpleExpr::from(tc.to_string()));
            has_values = true;
        }
        if let Some(amt) = update.amount {
            query.value(Alias::new("TRANSAMOUNT"), SimpleExpr::from(amt.v1));
            has_values = true;
        }
        if let Some(status) = update.status {
            query.value(Alias::new("STATUS"), SimpleExpr::from(status.to_string()));
            has_values = true;
        }
        if let Some(num) = update.transaction_number {
            query.value(Alias::new("TRANSACTIONNUMBER"), SimpleExpr::from(num));
            has_values = true;
        }
        if let Some(notes) = update.notes {
            query.value(Alias::new("NOTES"), SimpleExpr::from(notes));
            has_values = true;
        }
        if let Some(cat_id) = update.category_id {
            query.value(Alias::new("CATEGID"), SimpleExpr::from(cat_id.v1));
            has_values = true;
        }
        if let Some(date) = update.date {
            query.value(Alias::new("TRANSDATE"), SimpleExpr::from(date.v1));
            has_values = true;
        }
        if let Some(to_amt) = update.to_amount {
            query.value(Alias::new("TOTRANSAMOUNT"), SimpleExpr::from(to_amt.v1));
            has_values = true;
        }

        if !has_values {
            return Ok(());
        }

        query.and_where(Expr::col(Alias::new("TRANSID")).eq(id.v1));

        let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn delete(&self, id: TransactionId) -> Result<(), TransactionError> {
        let (sql, values) = Query::delete()
            .from_table(Alias::new("CHECKINGACCOUNT_V1"))
            .and_where(Expr::col(Alias::new("TRANSID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }
}
