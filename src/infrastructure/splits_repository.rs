use rusqlite::Row;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use rust_decimal::Decimal;
use std::str::FromStr;
use rust_decimal::prelude::FromPrimitive;

use crate::domain::transactions::{SplitTransaction, TransactionId, SplitRepository};
use crate::domain::types::Money;
use crate::domain::categories::CategoryId;
use crate::error::MmexError;
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
            transaction_id: TransactionId(row.get("TRANSID")?),
            category_id: row.get::<_, Option<i64>>("CATEGID")?.map(CategoryId),
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
    fn find_for_transaction(&self, tx_id: TransactionId) -> Result<Vec<SplitTransaction>, MmexError> {
        let (sql, _) = Query::select()
            .columns(["SPLITTRANSID", "TRANSID", "CATEGID", "SPLITTRANSAMOUNT", "NOTES"])
            .from_as("SPLITTRANSACTIONS_V1", "s")
            .and_where(Expr::col("TRANSID").eq(tx_id.0))
            .build(SqliteQueryBuilder);

        self.executor.query_map_ext(&sql, [tx_id.0], |row| SplitMapper::map_row(row))
    }

    fn insert(&self, s: &SplitTransaction) -> Result<SplitTransaction, MmexError> {
        let sql = "INSERT INTO SPLITTRANSACTIONS_V1 (TRANSID, CATEGID, SPLITTRANSAMOUNT, NOTES) VALUES (?, ?, ?, ?)";
        self.executor.execute_ext(sql, (s.transaction_id.0, s.category_id.map(|id| id.0), s.amount.0.to_string(), &s.notes))?;
        let last_id: i64 = self.executor.query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;
        let mut new_split = s.clone();
        new_split.id = last_id;
        Ok(new_split)
    }

    fn update(&self, s: &SplitTransaction) -> Result<(), MmexError> {
        let sql = "UPDATE SPLITTRANSACTIONS_V1 SET TRANSID = ?, CATEGID = ?, SPLITTRANSAMOUNT = ?, NOTES = ? WHERE SPLITTRANSID = ?";
        self.executor.execute_ext(sql, (s.transaction_id.0, s.category_id.map(|id| id.0), s.amount.0.to_string(), &s.notes, s.id))?;
        Ok(())
    }

    fn delete(&self, id: i64) -> Result<(), MmexError> {
        self.executor.execute_ext("DELETE FROM SPLITTRANSACTIONS_V1 WHERE SPLITTRANSID = ?", [id])?;
        Ok(())
    }

    fn delete_for_transaction(&self, tx_id: TransactionId) -> Result<(), MmexError> {
        self.executor.execute_ext("DELETE FROM SPLITTRANSACTIONS_V1 WHERE TRANSID = ?", [tx_id.0])?;
        Ok(())
    }
}
