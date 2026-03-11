use rusqlite::Row;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use std::str::FromStr;

use crate::domain::currencies::{Currency, CurrencyError, CurrencyId, CurrencyRepository};
use crate::infrastructure::db_executor::DbExecutor;
use crate::MmexError;

pub struct CurrencyMapper;

impl CurrencyMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Currency> {
        let base_conv_rate = if let Ok(val) = row.get::<_, f64>("BASECONVRATE") {
            Decimal::from_f64(val).unwrap_or(Decimal::ONE)
        } else if let Ok(s) = row.get::<_, String>("BASECONVRATE") {
            Decimal::from_str(&s).unwrap_or(Decimal::ONE)
        } else {
            Decimal::ONE
        };

        Ok(Currency {
            id: CurrencyId {
                v1: row.get("CURRENCYID")?,
            },
            name: row.get("CURRENCYNAME")?,
            pfx_symbol: row.get("PFX_SYMBOL")?,
            sfx_symbol: row.get("SFX_SYMBOL")?,
            decimal_point: row.get("DECIMAL_POINT")?,
            group_separator: row.get("GROUP_SEPARATOR")?,
            unit_name: row.get("UNIT_NAME")?,
            cent_name: row.get("CENT_NAME")?,
            scale: row.get("SCALE")?,
            base_conv_rate,
            symbol: row.get("CURRENCY_SYMBOL")?,
            currency_type: row.get("CURRENCY_TYPE")?,
        })
    }
}

pub struct SqlCurrencyRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlCurrencyRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> CurrencyRepository for SqlCurrencyRepository<'a, E> {
    fn find_all(&self) -> Result<Vec<Currency>, CurrencyError> {
        let (sql, _) = Query::select()
            .columns([
                "CURRENCYID",
                "CURRENCYNAME",
                "PFX_SYMBOL",
                "SFX_SYMBOL",
                "DECIMAL_POINT",
                "GROUP_SEPARATOR",
                "UNIT_NAME",
                "CENT_NAME",
                "SCALE",
                "BASECONVRATE",
                "CURRENCY_SYMBOL",
                "CURRENCY_TYPE",
            ])
            .from_as("CURRENCYFORMATS_V1", "c")
            .build(SqliteQueryBuilder);
        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| CurrencyMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: CurrencyId) -> Result<Option<Currency>, CurrencyError> {
        let (sql, _) = Query::select()
            .columns([
                "CURRENCYID",
                "CURRENCYNAME",
                "PFX_SYMBOL",
                "SFX_SYMBOL",
                "DECIMAL_POINT",
                "GROUP_SEPARATOR",
                "UNIT_NAME",
                "CENT_NAME",
                "SCALE",
                "BASECONVRATE",
                "CURRENCY_SYMBOL",
                "CURRENCY_TYPE",
            ])
            .from_as("CURRENCYFORMATS_V1", "c")
            .and_where(Expr::col("CURRENCYID").eq(id.v1))
            .build(SqliteQueryBuilder);
        match self
            .executor
            .query_row_ext(&sql, [id.v1], |row| CurrencyMapper::map_row(row))
        {
            Ok(curr) => Ok(Some(curr)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(CurrencyError::Common(e)),
        }
    }

    fn find_by_symbol(&self, symbol: &str) -> Result<Option<Currency>, CurrencyError> {
        let (sql, _) = Query::select()
            .columns([
                "CURRENCYID",
                "CURRENCYNAME",
                "PFX_SYMBOL",
                "SFX_SYMBOL",
                "DECIMAL_POINT",
                "GROUP_SEPARATOR",
                "UNIT_NAME",
                "CENT_NAME",
                "SCALE",
                "BASECONVRATE",
                "CURRENCY_SYMBOL",
                "CURRENCY_TYPE",
            ])
            .from_as("CURRENCYFORMATS_V1", "c")
            .and_where(Expr::col("CURRENCY_SYMBOL").eq(symbol))
            .build(SqliteQueryBuilder);
        match self
            .executor
            .query_row_ext(&sql, [symbol], |row| CurrencyMapper::map_row(row))
        {
            Ok(curr) => Ok(Some(curr)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(CurrencyError::Common(e)),
        }
    }

    fn insert(&self, c: &Currency) -> Result<Currency, CurrencyError> {
        let sql = "INSERT INTO CURRENCYFORMATS_V1 (CURRENCYNAME, PFX_SYMBOL, SFX_SYMBOL, DECIMAL_POINT, GROUP_SEPARATOR, UNIT_NAME, CENT_NAME, SCALE, BASECONVRATE, CURRENCY_SYMBOL, CURRENCY_TYPE) 
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        self.executor.execute_ext(
            sql,
            (
                &c.name,
                &c.pfx_symbol,
                &c.sfx_symbol,
                &c.decimal_point,
                &c.group_separator,
                &c.unit_name,
                &c.cent_name,
                c.scale,
                c.base_conv_rate.to_string(),
                &c.symbol,
                &c.currency_type,
            ),
        )?;
        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;
        let mut new_curr = c.clone();
        new_curr.id = CurrencyId { v1: last_id };
        Ok(new_curr)
    }

    fn update(&self, c: &Currency) -> Result<(), CurrencyError> {
        let sql = "UPDATE CURRENCYFORMATS_V1 SET 
                   CURRENCYNAME = ?, PFX_SYMBOL = ?, SFX_SYMBOL = ?, DECIMAL_POINT = ?, GROUP_SEPARATOR = ?, UNIT_NAME = ?, CENT_NAME = ?, SCALE = ?, BASECONVRATE = ?, CURRENCY_SYMBOL = ?, CURRENCY_TYPE = ?
                   WHERE CURRENCYID = ?";
        self.executor.execute_ext(
            sql,
            (
                &c.name,
                &c.pfx_symbol,
                &c.sfx_symbol,
                &c.decimal_point,
                &c.group_separator,
                &c.unit_name,
                &c.cent_name,
                c.scale,
                c.base_conv_rate.to_string(),
                &c.symbol,
                &c.currency_type,
                c.id.v1,
            ),
        )?;
        Ok(())
    }

    fn delete(&self, id: CurrencyId) -> Result<(), CurrencyError> {
        self.executor.execute_ext(
            "DELETE FROM CURRENCYFORMATS_V1 WHERE CURRENCYID = ?",
            [id.v1],
        )?;
        Ok(())
    }
}
