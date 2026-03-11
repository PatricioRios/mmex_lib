use chrono::NaiveDate;
use rusqlite::Row;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use std::str::FromStr;

use crate::domain::payees::PayeeId;
use crate::domain::scheduled_transactions::{
    ScheduledError, ScheduledRepository, ScheduledTransaction,
};
use crate::domain::transactions::{TransactionCode, TransactionStatus};
use crate::domain::types::{AccountId, CategoryId, Money};
use crate::infrastructure::db_executor::DbExecutor;
use crate::MmexError;

pub struct ScheduledMapper;

impl ScheduledMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<ScheduledTransaction> {
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
        let trans_date = date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

        let next_date_str: Option<String> = row.get("NEXTOCCURRENCEDATE")?;
        let next_occurrence_date =
            next_date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

        Ok(ScheduledTransaction {
            id: row.get("BDID")?,
            account_id: AccountId {
                v1: row.get("ACCOUNTID")?,
            },
            to_account_id: row
                .get::<_, Option<i64>>("TOACCOUNTID")?
                .map(|v1| AccountId { v1 }),
            payee_id: PayeeId {
                v1: row.get("PAYEEID")?,
            },
            trans_code: TransactionCode::from(row.get::<_, String>("TRANSCODE")?),
            amount: Money(amount_val),
            status: TransactionStatus::from(row.get::<_, String>("STATUS")?),
            transaction_number: row.get("TRANSACTIONNUMBER")?,
            notes: row.get("NOTES")?,
            category_id: row
                .get::<_, Option<i64>>("CATEGID")?
                .map(|v1| CategoryId { v1 }),
            trans_date,
            next_occurrence_date,
            repeats: row.get("REPEATS")?,
            num_occurrences: row.get("NUMOCCURRENCES")?,
            to_trans_amount: to_amount_val,
        })
    }
}

pub struct SqlScheduledRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlScheduledRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> ScheduledRepository for SqlScheduledRepository<'a, E> {
    fn find_all(&self) -> Result<Vec<ScheduledTransaction>, ScheduledError> {
        let (sql, _) = Query::select()
            .columns([
                "BDID",
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
                "REPEATS",
                "NEXTOCCURRENCEDATE",
                "NUMOCCURRENCES",
            ])
            .from_as("BILLSDEPOSITS_V1", "b")
            .build(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| ScheduledMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: i64) -> Result<Option<ScheduledTransaction>, ScheduledError> {
        let (sql, _) = Query::select()
            .columns([
                "BDID",
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
                "REPEATS",
                "NEXTOCCURRENCEDATE",
                "NUMOCCURRENCES",
            ])
            .from_as("BILLSDEPOSITS_V1", "b")
            .and_where(Expr::col("BDID").eq(id))
            .build(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, [id], |row| ScheduledMapper::map_row(row))
        {
            Ok(tx) => Ok(Some(tx)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(ScheduledError::Common(e)),
        }
    }

    fn insert(&self, s: &ScheduledTransaction) -> Result<ScheduledTransaction, ScheduledError> {
        let trans_date_str = s.trans_date.map(|d| d.to_string());
        let next_date_str = s.next_occurrence_date.map(|d| d.to_string());

        let sql = "INSERT INTO BILLSDEPOSITS_V1 (ACCOUNTID, TOACCOUNTID, PAYEEID, TRANSCODE, TRANSAMOUNT, STATUS, TRANSACTIONNUMBER, NOTES, CATEGID, TRANSDATE, TOTRANSAMOUNT, REPEATS, NEXTOCCURRENCEDATE, NUMOCCURRENCES) 
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        self.executor.execute_ext(
            sql,
            (
                s.account_id.v1,
                s.to_account_id.map(|id| id.v1),
                s.payee_id.v1,
                s.trans_code.to_string(),
                s.amount.0.to_string(),
                s.status.to_string(),
                &s.transaction_number,
                &s.notes,
                s.category_id.map(|id| id.v1),
                trans_date_str,
                s.to_trans_amount.as_ref().map(|m| m.0.to_string()),
                s.repeats,
                next_date_str,
                s.num_occurrences,
            ),
        )?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;
        let mut new_tx = s.clone();
        new_tx.id = last_id;
        Ok(new_tx)
    }

    fn update(&self, s: &ScheduledTransaction) -> Result<(), ScheduledError> {
        let trans_date_str = s.trans_date.map(|d| d.to_string());
        let next_date_str = s.next_occurrence_date.map(|d| d.to_string());

        let sql = "UPDATE BILLSDEPOSITS_V1 SET 
                   ACCOUNTID = ?, TOACCOUNTID = ?, PAYEEID = ?, TRANSCODE = ?, TRANSAMOUNT = ?, STATUS = ?, TRANSACTIONNUMBER = ?, NOTES = ?, CATEGID = ?, TRANSDATE = ?, TOTRANSAMOUNT = ?, REPEATS = ?, NEXTOCCURRENCEDATE = ?, NUMOCCURRENCES = ?
                   WHERE BDID = ?";

        self.executor.execute_ext(
            sql,
            (
                s.account_id.v1,
                s.to_account_id.map(|id| id.v1),
                s.payee_id.v1,
                s.trans_code.to_string(),
                s.amount.0.to_string(),
                s.status.to_string(),
                &s.transaction_number,
                &s.notes,
                s.category_id.map(|id| id.v1),
                trans_date_str,
                s.to_trans_amount.as_ref().map(|m| m.0.to_string()),
                s.repeats,
                next_date_str,
                s.num_occurrences,
                s.id,
            ),
        )?;
        Ok(())
    }

    fn delete(&self, id: i64) -> Result<(), ScheduledError> {
        self.executor
            .execute_ext("DELETE FROM BILLSDEPOSITS_V1 WHERE BDID = ?", [id])?;
        Ok(())
    }
}
