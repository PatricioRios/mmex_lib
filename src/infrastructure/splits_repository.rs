use rusqlite::Row;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use std::str::FromStr;

use crate::domain::transactions::{
    SplitRepository, SplitTransaction, TransactionError, TransactionId,
};
use crate::domain::types::{CategoryId, Money};
use crate::infrastructure::db_executor::DbExecutor;

pub struct SplitMapper;

impl SplitMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<SplitTransaction> {
        let amount_val = if let Ok(val) = row.get::<_, f64>("SPLITTRANSAMOUNT") {
            Decimal::from_f64(val).unwrap_or(Decimal::ZERO)
        } else if let Ok(s) = row.get::<_, String>("SPLITTRANSAMOUNT") {
            Decimal::from_str(&s).unwrap_or(Decimal::ZERO)
        } else {
            Decimal::ZERO
        };

        Ok(SplitTransaction {
            id: row.get("SPLITTRANSID")?,
            transaction_id: TransactionId {
                v1: row.get("TRANSID")?,
            },
            category_id: row
                .get::<_, Option<i64>>("CATEGID")?
                .map(|v1| CategoryId { v1 }),
            amount: Money(amount_val),
            notes: row.get("NOTES")?,
        })
    }
}

pub struct SqlSplitRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlSplitRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> SplitRepository for SqlSplitRepository<'a, E> {
    fn find_for_transaction(
        &self,
        tx_id: TransactionId,
    ) -> Result<Vec<SplitTransaction>, TransactionError> {
        let (sql, _) = Query::select()
            .columns([
                "SPLITTRANSID",
                "TRANSID",
                "CATEGID",
                "SPLITTRANSAMOUNT",
                "NOTES",
            ])
            .from_as("SPLITTRANSACTIONS_V1", "s")
            .and_where(Expr::col("TRANSID").eq(tx_id.v1))
            .build(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, [tx_id.v1], |row| SplitMapper::map_row(row))?)
    }

    fn insert(&self, s: &SplitTransaction) -> Result<SplitTransaction, TransactionError> {
        let sql = "INSERT INTO SPLITTRANSACTIONS_V1 (TRANSID, CATEGID, SPLITTRANSAMOUNT, NOTES) VALUES (?, ?, ?, ?)";
        self.executor.execute_ext(
            sql,
            (
                s.transaction_id.v1,
                s.category_id.map(|id| id.v1),
                s.amount.0.to_string(),
                &s.notes,
            ),
        )?;
        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;
        let mut new_split = s.clone();
        new_split.id = last_id;
        Ok(new_split)
    }

    fn update(&self, s: &SplitTransaction) -> Result<(), TransactionError> {
        let sql = "UPDATE SPLITTRANSACTIONS_V1 SET TRANSID = ?, CATEGID = ?, SPLITTRANSAMOUNT = ?, NOTES = ? WHERE SPLITTRANSID = ?";
        self.executor.execute_ext(
            sql,
            (
                s.transaction_id.v1,
                s.category_id.map(|id| id.v1),
                s.amount.0.to_string(),
                &s.notes,
                s.id,
            ),
        )?;
        Ok(())
    }

    fn delete(&self, id: i64) -> Result<(), TransactionError> {
        self.executor.execute_ext(
            "DELETE FROM SPLITTRANSACTIONS_V1 WHERE SPLITTRANSID = ?",
            [id],
        )?;
        Ok(())
    }

    fn delete_for_transaction(&self, tx_id: TransactionId) -> Result<(), TransactionError> {
        self.executor.execute_ext(
            "DELETE FROM SPLITTRANSACTIONS_V1 WHERE TRANSID = ?",
            [tx_id.v1],
        )?;
        Ok(())
    }
}
