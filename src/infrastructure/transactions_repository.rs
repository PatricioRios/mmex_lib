use rusqlite::Row;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use rust_decimal::Decimal;
use std::str::FromStr;
use rust_decimal::prelude::FromPrimitive;
use chrono::NaiveDate;

use crate::domain::transactions::{Transaction, TransactionId, TransactionCode, TransactionStatus, TransactionRepository};
use crate::domain::types::{AccountId, Money, CategoryId};
use crate::domain::payees::PayeeId;
use crate::error::MmexError;
use crate::infrastructure::db_executor::DbExecutor;

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
            Some(Money(Decimal::from_f64(val).unwrap_or(Decimal::ZERO)))
        } else if let Ok(s) = row.get::<_, String>("TOTRANSAMOUNT") {
            Some(Money(Decimal::from_str(&s).unwrap_or(Decimal::ZERO)))
        } else {
            None
        };

        let date_str: Option<String> = row.get("TRANSDATE")?;
        let date = date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

        Ok(Transaction {
            id: TransactionId(row.get("TRANSID")?),
            account_id: AccountId(row.get("ACCOUNTID")?),
            to_account_id: row.get::<_, Option<i64>>("TOACCOUNTID")?.map(AccountId),
            payee_id: PayeeId(row.get("PAYEEID")?),
            trans_code: TransactionCode::from(row.get::<_, String>("TRANSCODE")?),
            amount: Money(amount_val),
            status: TransactionStatus::from(row.get::<_, String>("STATUS")?),
            transaction_number: row.get("TRANSACTIONNUMBER")?,
            notes: row.get("NOTES")?,
            category_id: row.get::<_, Option<i64>>("CATEGID")?.map(CategoryId),
            date,
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
    fn find_all(&self) -> Result<Vec<Transaction>, MmexError> {
        let (sql, _) = Query::select()
            .columns(["TRANSID", "ACCOUNTID", "TOACCOUNTID", "PAYEEID", "TRANSCODE", "TRANSAMOUNT", "STATUS", "TRANSACTIONNUMBER", "NOTES", "CATEGID", "TRANSDATE", "TOTRANSAMOUNT"])
            .from_as("CHECKINGACCOUNT_V1", "t")
            .build(SqliteQueryBuilder);
        self.executor.query_map_ext(&sql, [], |row| TransactionMapper::map_row(row))
    }

    fn find_by_id(&self, id: TransactionId) -> Result<Option<Transaction>, MmexError> {
        let (sql, _) = Query::select()
            .columns(["TRANSID", "ACCOUNTID", "TOACCOUNTID", "PAYEEID", "TRANSCODE", "TRANSAMOUNT", "STATUS", "TRANSACTIONNUMBER", "NOTES", "CATEGID", "TRANSDATE", "TOTRANSAMOUNT"])
            .from_as("CHECKINGACCOUNT_V1", "t")
            .and_where(Expr::col("TRANSID").eq(id.0))
            .build(SqliteQueryBuilder);
        match self.executor.query_row_ext(&sql, [id.0], |row| TransactionMapper::map_row(row)) {
            Ok(tx) => Ok(Some(tx)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn insert(&self, tx: &Transaction) -> Result<Transaction, MmexError> {
        let date_str = tx.date.map(|d| d.to_string());
        let sql = "INSERT INTO CHECKINGACCOUNT_V1 (ACCOUNTID, TOACCOUNTID, PAYEEID, TRANSCODE, TRANSAMOUNT, STATUS, TRANSACTIONNUMBER, NOTES, CATEGID, TRANSDATE, TOTRANSAMOUNT) 
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        self.executor.execute_ext(sql, (tx.account_id.0, tx.to_account_id.map(|id| id.0), tx.payee_id.0, tx.trans_code.to_string(), tx.amount.0.to_string(), tx.status.to_string(), &tx.transaction_number, &tx.notes, tx.category_id.map(|id| id.0), date_str, tx.to_amount.as_ref().map(|m| m.0.to_string())))?;
        let last_id: i64 = self.executor.query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;
        let mut new_tx = tx.clone();
        new_tx.id = TransactionId(last_id);
        Ok(new_tx)
    }

    fn update(&self, tx: &Transaction) -> Result<(), MmexError> {
        let date_str = tx.date.map(|d| d.to_string());
        let sql = "UPDATE CHECKINGACCOUNT_V1 SET 
                   ACCOUNTID = ?, TOACCOUNTID = ?, PAYEEID = ?, TRANSCODE = ?, TRANSAMOUNT = ?, STATUS = ?, TRANSACTIONNUMBER = ?, NOTES = ?, CATEGID = ?, TRANSDATE = ?, TOTRANSAMOUNT = ?
                   WHERE TRANSID = ?";
        self.executor.execute_ext(sql, (tx.account_id.0, tx.to_account_id.map(|id| id.0), tx.payee_id.0, tx.trans_code.to_string(), tx.amount.0.to_string(), tx.status.to_string(), &tx.transaction_number, &tx.notes, tx.category_id.map(|id| id.0), date_str, tx.to_amount.as_ref().map(|m| m.0.to_string()), tx.id.0))?;
        Ok(())
    }

    fn delete(&self, id: TransactionId) -> Result<(), MmexError> {
        self.executor.execute_ext("DELETE FROM CHECKINGACCOUNT_V1 WHERE TRANSID = ?", [id.0])?;
        Ok(())
    }
}
