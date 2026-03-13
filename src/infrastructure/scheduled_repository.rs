use chrono::NaiveDate;
use rusqlite::Row;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use std::str::FromStr;

use crate::domain::payees::PayeeId;
use crate::domain::scheduled_transactions::{
    ScheduledError, ScheduledRepository, ScheduledTransaction,
};
use crate::domain::transactions::{TransactionCode, TransactionStatus};
use crate::domain::types::{AccountId, CategoryId, MmexDate, Money};
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
            Some(Money::from(Decimal::from_f64(val).unwrap_or(Decimal::ZERO)))
        } else if let Ok(s) = row.get::<_, String>("TOTRANSAMOUNT") {
            Some(Money::from(Decimal::from_str(&s).unwrap_or(Decimal::ZERO)))
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
            trans_date: trans_date.map(MmexDate::from),
            next_occurrence_date: next_occurrence_date.map(MmexDate::from),
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
            .from(Alias::new("BILLSDEPOSITS_V1"))
            .build(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| ScheduledMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: i64) -> Result<Option<ScheduledTransaction>, ScheduledError> {
        let (sql, values) = Query::select()
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
            .from(Alias::new("BILLSDEPOSITS_V1"))
            .and_where(Expr::col(Alias::new("BDID")).eq(id))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |row| {
                ScheduledMapper::map_row(row)
            }) {
            Ok(tx) => Ok(Some(tx)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(ScheduledError::Common(e)),
        }
    }

    fn insert(&self, s: &ScheduledTransaction) -> Result<ScheduledTransaction, ScheduledError> {
        let trans_date_str = s.trans_date.as_ref().map(|d| d.v1.clone());
        let next_date_str = s.next_occurrence_date.as_ref().map(|d| d.v1.clone());

        let (sql, values) = Query::insert()
            .into_table(Alias::new("BILLSDEPOSITS_V1"))
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
                Alias::new("REPEATS"),
                Alias::new("NEXTOCCURRENCEDATE"),
                Alias::new("NUMOCCURRENCES"),
            ])
            .values_panic([
                s.account_id.v1.into(),
                s.to_account_id.map(|id| id.v1).into(),
                s.payee_id.v1.into(),
                s.trans_code.to_string().into(),
                s.amount.v1.clone().into(),
                s.status.to_string().into(),
                s.transaction_number.clone().into(),
                s.notes.clone().into(),
                s.category_id.map(|id| id.v1).into(),
                trans_date_str.into(),
                s.to_trans_amount.as_ref().map(|m| m.v1.clone()).into(),
                s.repeats.into(),
                next_date_str.into(),
                s.num_occurrences.into(),
            ])
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;

        let mut new_tx = s.clone();
        new_tx.id = last_id;
        Ok(new_tx)
    }

    fn update(&self, s: &ScheduledTransaction) -> Result<(), ScheduledError> {
        let trans_date_str = s.trans_date.as_ref().map(|d| d.v1.clone());
        let next_date_str = s.next_occurrence_date.as_ref().map(|d| d.v1.clone());

        let (sql, values) = Query::update()
            .table(Alias::new("BILLSDEPOSITS_V1"))
            .values([
                (Alias::new("ACCOUNTID"), s.account_id.v1.into()),
                (
                    Alias::new("TOACCOUNTID"),
                    s.to_account_id.map(|id| id.v1).into(),
                ),
                (Alias::new("PAYEEID"), s.payee_id.v1.into()),
                (Alias::new("TRANSCODE"), s.trans_code.to_string().into()),
                (Alias::new("TRANSAMOUNT"), s.amount.v1.clone().into()),
                (Alias::new("STATUS"), s.status.to_string().into()),
                (
                    Alias::new("TRANSACTIONNUMBER"),
                    s.transaction_number.clone().into(),
                ),
                (Alias::new("NOTES"), s.notes.clone().into()),
                (Alias::new("CATEGID"), s.category_id.map(|id| id.v1).into()),
                (Alias::new("TRANSDATE"), trans_date_str.into()),
                (
                    Alias::new("TOTRANSAMOUNT"),
                    s.to_trans_amount.as_ref().map(|m| m.v1.clone()).into(),
                ),
                (Alias::new("REPEATS"), s.repeats.into()),
                (Alias::new("NEXTOCCURRENCEDATE"), next_date_str.into()),
                (Alias::new("NUMOCCURRENCES"), s.num_occurrences.into()),
            ])
            .and_where(Expr::col(Alias::new("BDID")).eq(s.id))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn delete(&self, id: i64) -> Result<(), ScheduledError> {
        let (sql, values) = Query::delete()
            .from_table(Alias::new("BILLSDEPOSITS_V1"))
            .and_where(Expr::col(Alias::new("BDID")).eq(id))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }
}
