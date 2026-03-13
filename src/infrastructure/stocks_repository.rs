use chrono::NaiveDate;
use rusqlite::Row;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use std::str::FromStr;

use crate::domain::stocks::{Stock, StockError, StockId, StockRepository};
use crate::domain::types::{MmexDate, Money};
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
            id: StockId::new(row.get("STOCKID")?),
            held_at: row.get("HELDAT")?,
            purchase_date: MmexDate::from(purchase_date),
            name: row.get("STOCKNAME")?,
            symbol: row.get("SYMBOL")?,
            num_shares: Money::from(Self::parse_decimal(row, "NUMSHARES")),
            purchase_price: Money::from(Self::parse_decimal(row, "PURCHASEPRICE")),
            notes: row.get("NOTES")?,
            current_price: Money::from(Self::parse_decimal(row, "CURRENTPRICE")),
            value: Money::from(Self::parse_decimal(row, "VALUE")),
            commission: Money::from(Self::parse_decimal(row, "COMMISSION")),
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
            .from(Alias::new("STOCK_V1"))
            .build(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| StockMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: StockId) -> Result<Option<Stock>, StockError> {
        let (sql, values) = Query::select()
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
            .from(Alias::new("STOCK_V1"))
            .and_where(Expr::col(Alias::new("STOCKID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |row| {
                StockMapper::map_row(row)
            }) {
            Ok(stock) => Ok(Some(stock)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(StockError::Common(e)),
        }
    }

    fn insert(&self, s: &Stock) -> Result<Stock, StockError> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("STOCK_V1"))
            .columns([
                Alias::new("HELDAT"),
                Alias::new("PURCHASEDATE"),
                Alias::new("STOCKNAME"),
                Alias::new("SYMBOL"),
                Alias::new("NUMSHARES"),
                Alias::new("PURCHASEPRICE"),
                Alias::new("NOTES"),
                Alias::new("CURRENTPRICE"),
                Alias::new("VALUE"),
                Alias::new("COMMISSION"),
            ])
            .values_panic([
                s.held_at.into(),
                s.purchase_date.v1.clone().into(),
                s.name.clone().into(),
                s.symbol.clone().into(),
                s.num_shares.v1.clone().into(),
                s.purchase_price.v1.clone().into(),
                s.notes.clone().into(),
                s.current_price.v1.clone().into(),
                s.value.v1.clone().into(),
                s.commission.v1.clone().into(),
            ])
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;

        let mut new_stock = s.clone();
        new_stock.id = StockId::new(last_id);
        Ok(new_stock)
    }

    fn update(&self, s: &Stock) -> Result<(), StockError> {
        let (sql, values) = Query::update()
            .table(Alias::new("STOCK_V1"))
            .values([
                (Alias::new("HELDAT"), s.held_at.into()),
                (
                    Alias::new("PURCHASEDATE"),
                    s.purchase_date.v1.clone().into(),
                ),
                (Alias::new("STOCKNAME"), s.name.clone().into()),
                (Alias::new("SYMBOL"), s.symbol.clone().into()),
                (Alias::new("NUMSHARES"), s.num_shares.v1.clone().into()),
                (
                    Alias::new("PURCHASEPRICE"),
                    s.purchase_price.v1.clone().into(),
                ),
                (Alias::new("NOTES"), s.notes.clone().into()),
                (
                    Alias::new("CURRENTPRICE"),
                    s.current_price.v1.clone().into(),
                ),
                (Alias::new("VALUE"), s.value.v1.clone().into()),
                (Alias::new("COMMISSION"), s.commission.v1.clone().into()),
            ])
            .and_where(Expr::col(Alias::new("STOCKID")).eq(s.id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn delete(&self, id: StockId) -> Result<(), StockError> {
        let (sql, values) = Query::delete()
            .from_table(Alias::new("STOCK_V1"))
            .and_where(Expr::col(Alias::new("STOCKID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }
}
