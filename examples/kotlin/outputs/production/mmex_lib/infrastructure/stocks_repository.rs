use chrono::NaiveDate;
use rusqlite::Row;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use std::str::FromStr;

use crate::domain::stocks::{Stock, StockError, StockId, StockRepository};
use crate::domain::types::Money;
use crate::infrastructure::db_executor::DbExecutor;
use crate::MmexError;

pub struct StockMapper;

impl StockMapper {
    fn parse_decimal(row: &Row, column: &str) -> Decimal {
        if let Ok(val) = row.get::<_, f64>(column) {
            Decimal::from_f64(val).unwrap_or(Decimal::ZERO)
        } else if let Ok(s) = row.get::<_, String>(column) {
            Decimal::from_str(&s).unwrap_or(Decimal::ZERO)
        } else {
            Decimal::ZERO
        }
    }

    pub fn map_row(row: &Row) -> rusqlite::Result<Stock> {
        let purchase_date_str: String = row.get("PURCHASEDATE")?;
        let purchase_date = NaiveDate::parse_from_str(&purchase_date_str, "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

        Ok(Stock {
            id: StockId(row.get("STOCKID")?),
            held_at: row.get("HELDAT")?,
            purchase_date,
            name: row.get("STOCKNAME")?,
            symbol: row.get("SYMBOL")?,
            num_shares: Self::parse_decimal(row, "NUMSHARES"),
            purchase_price: Money(Self::parse_decimal(row, "PURCHASEPRICE")),
            notes: row.get("NOTES")?,
            current_price: Money(Self::parse_decimal(row, "CURRENTPRICE")),
            value: Money(Self::parse_decimal(row, "VALUE")),
            commission: Money(Self::parse_decimal(row, "COMMISSION")),
        })
    }
}

pub struct SqlStockRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlStockRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> StockRepository for SqlStockRepository<'a, E> {
    fn find_all(&self) -> Result<Vec<Stock>, StockError> {
        let (sql, _) = Query::select()
            .columns([
                "STOCKID",
                "HELDAT",
                "PURCHASEDATE",
                "STOCKNAME",
                "SYMBOL",
                "NUMSHARES",
                "PURCHASEPRICE",
                "NOTES",
                "CURRENTPRICE",
                "VALUE",
                "COMMISSION",
            ])
            .from_as("STOCK_V1", "s")
            .build(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| StockMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: StockId) -> Result<Option<Stock>, StockError> {
        let (sql, _) = Query::select()
            .columns([
                "STOCKID",
                "HELDAT",
                "PURCHASEDATE",
                "STOCKNAME",
                "SYMBOL",
                "NUMSHARES",
                "PURCHASEPRICE",
                "NOTES",
                "CURRENTPRICE",
                "VALUE",
                "COMMISSION",
            ])
            .from_as("STOCK_V1", "s")
            .and_where(Expr::col("STOCKID").eq(id.0))
            .build(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, [id.0], |row| StockMapper::map_row(row))
        {
            Ok(stock) => Ok(Some(stock)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(StockError::Common(e)),
        }
    }

    fn insert(&self, s: &Stock) -> Result<Stock, StockError> {
        let sql = "INSERT INTO STOCK_V1 (HELDAT, PURCHASEDATE, STOCKNAME, SYMBOL, NUMSHARES, PURCHASEPRICE, NOTES, CURRENTPRICE, VALUE, COMMISSION) 
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        self.executor.execute_ext(
            sql,
            (
                s.held_at,
                s.purchase_date.to_string(),
                &s.name,
                &s.symbol,
                s.num_shares.to_string(),
                s.purchase_price.0.to_string(),
                &s.notes,
                s.current_price.0.to_string(),
                s.value.0.to_string(),
                s.commission.0.to_string(),
            ),
        )?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;
        let mut new_stock = s.clone();
        new_stock.id = StockId(last_id);
        Ok(new_stock)
    }

    fn update(&self, s: &Stock) -> Result<(), StockError> {
        let sql = "UPDATE STOCK_V1 SET 
                   HELDAT = ?, PURCHASEDATE = ?, STOCKNAME = ?, SYMBOL = ?, NUMSHARES = ?, PURCHASEPRICE = ?, NOTES = ?, CURRENTPRICE = ?, VALUE = ?, COMMISSION = ?
                   WHERE STOCKID = ?";

        self.executor.execute_ext(
            sql,
            (
                s.held_at,
                s.purchase_date.to_string(),
                &s.name,
                &s.symbol,
                s.num_shares.to_string(),
                s.purchase_price.0.to_string(),
                &s.notes,
                s.current_price.0.to_string(),
                s.value.0.to_string(),
                s.commission.0.to_string(),
                s.id.0,
            ),
        )?;
        Ok(())
    }

    fn delete(&self, id: StockId) -> Result<(), StockError> {
        self.executor
            .execute_ext("DELETE FROM STOCK_V1 WHERE STOCKID = ?", [id.0])?;
        Ok(())
    }
}
