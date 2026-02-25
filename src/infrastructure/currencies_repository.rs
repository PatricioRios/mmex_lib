use rusqlite::Row;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use std::str::FromStr;

use crate::domain::currencies::{Currency, CurrencyId, CurrencyRepository};
use crate::error::MmexError;
use crate::infrastructure::db_executor::DbExecutor;

pub struct CurrencyMapper;

impl CurrencyMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Currency> {
        // MMEX puede guardar numeric como Real o como String. 
        // Intentamos f64 primero ya que rusqlite dio error de tipo.
        let base_conv_rate = if let Ok(val) = row.get::<_, f64>("BASECONVRATE") {
            Decimal::from_f64(val).unwrap_or(Decimal::ONE)
        } else if let Ok(s) = row.get::<_, String>("BASECONVRATE") {
            Decimal::from_str(&s).unwrap_or(Decimal::ONE)
        } else {
            Decimal::ONE
        };

        Ok(Currency {
            id: CurrencyId(row.get("CURRENCYID")?),
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
    fn find_all(&self) -> Result<Vec<Currency>, MmexError> {
        let (sql, _) = Query::select()
            .columns([
                "CURRENCYID", "CURRENCYNAME", "PFX_SYMBOL", "SFX_SYMBOL",
                "DECIMAL_POINT", "GROUP_SEPARATOR", "UNIT_NAME", "CENT_NAME",
                "SCALE", "BASECONVRATE", "CURRENCY_SYMBOL", "CURRENCY_TYPE"
            ])
            .from_as("CURRENCYFORMATS_V1", "c")
            .build(SqliteQueryBuilder);

        self.executor.query_map_ext(&sql, [], |row| CurrencyMapper::map_row(row))
    }

    fn find_by_id(&self, id: CurrencyId) -> Result<Option<Currency>, MmexError> {
        let (sql, _) = Query::select()
            .columns([
                "CURRENCYID", "CURRENCYNAME", "PFX_SYMBOL", "SFX_SYMBOL",
                "DECIMAL_POINT", "GROUP_SEPARATOR", "UNIT_NAME", "CENT_NAME",
                "SCALE", "BASECONVRATE", "CURRENCY_SYMBOL", "CURRENCY_TYPE"
            ])
            .from_as("CURRENCYFORMATS_V1", "c")
            .and_where(Expr::col("CURRENCYID").eq(id.0))
            .build(SqliteQueryBuilder);

        match self.executor.query_row_ext(&sql, [id.0], |row| CurrencyMapper::map_row(row)) {
            Ok(curr) => Ok(Some(curr)),
            Err(MmexError::Database(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn find_by_symbol(&self, symbol: &str) -> Result<Option<Currency>, MmexError> {
        let (sql, _) = Query::select()
            .columns([
                "CURRENCYID", "CURRENCYNAME", "PFX_SYMBOL", "SFX_SYMBOL",
                "DECIMAL_POINT", "GROUP_SEPARATOR", "UNIT_NAME", "CENT_NAME",
                "SCALE", "BASECONVRATE", "CURRENCY_SYMBOL", "CURRENCY_TYPE"
            ])
            .from_as("CURRENCYFORMATS_V1", "c")
            .and_where(Expr::col("CURRENCY_SYMBOL").eq(symbol))
            .build(SqliteQueryBuilder);

        match self.executor.query_row_ext(&sql, [symbol], |row| CurrencyMapper::map_row(row)) {
            Ok(curr) => Ok(Some(curr)),
            Err(MmexError::Database(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
