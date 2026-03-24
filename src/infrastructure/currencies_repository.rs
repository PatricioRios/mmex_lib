use rusqlite::Row;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_query::{Alias, Expr, Query, SimpleExpr, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use std::str::FromStr;

use crate::domain::currencies::{Currency, CurrencyError, CurrencyId, CurrencyRepository};
use crate::domain::types::Money;
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
            id: CurrencyId::new(row.get("CURRENCYID")?),
            name: row.get("CURRENCYNAME")?,
            pfx_symbol: row.get("PFX_SYMBOL")?,
            sfx_symbol: row.get("SFX_SYMBOL")?,
            decimal_point: row.get("DECIMAL_POINT")?,
            group_separator: row.get("GROUP_SEPARATOR")?,
            unit_name: row.get("UNIT_NAME")?,
            cent_name: row.get("CENT_NAME")?,
            scale: row.get("SCALE")?,
            base_conv_rate: Money::from(base_conv_rate),
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
            .from(Alias::new("CURRENCYFORMATS_V1"))
            .build(SqliteQueryBuilder);
        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| CurrencyMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: CurrencyId) -> Result<Option<Currency>, CurrencyError> {
        let (sql, values) = Query::select()
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
            .from(Alias::new("CURRENCYFORMATS_V1"))
            .and_where(Expr::col(Alias::new("CURRENCYID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |row| {
                CurrencyMapper::map_row(row)
            }) {
            Ok(curr) => Ok(Some(curr)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(CurrencyError::Common(e)),
        }
    }

    fn find_by_symbol(&self, symbol: &str) -> Result<Option<Currency>, CurrencyError> {
        let (sql, values) = Query::select()
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
            .from(Alias::new("CURRENCYFORMATS_V1"))
            .and_where(Expr::col(Alias::new("CURRENCY_SYMBOL")).eq(symbol))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |row| {
                CurrencyMapper::map_row(row)
            }) {
            Ok(curr) => Ok(Some(curr)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(CurrencyError::Common(e)),
        }
    }

    fn insert(&self, c: &Currency) -> Result<Currency, CurrencyError> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("CURRENCYFORMATS_V1"))
            .columns([
                Alias::new("CURRENCYNAME"),
                Alias::new("PFX_SYMBOL"),
                Alias::new("SFX_SYMBOL"),
                Alias::new("DECIMAL_POINT"),
                Alias::new("GROUP_SEPARATOR"),
                Alias::new("UNIT_NAME"),
                Alias::new("CENT_NAME"),
                Alias::new("SCALE"),
                Alias::new("BASECONVRATE"),
                Alias::new("CURRENCY_SYMBOL"),
                Alias::new("CURRENCY_TYPE"),
            ])
            .values_panic([
                c.name.clone().into(),
                c.pfx_symbol.clone().into(),
                c.sfx_symbol.clone().into(),
                c.decimal_point.clone().into(),
                c.group_separator.clone().into(),
                c.unit_name.clone().into(),
                c.cent_name.clone().into(),
                c.scale.into(),
                c.base_conv_rate.v1.clone().into(),
                c.symbol.clone().into(),
                c.currency_type.clone().into(),
            ])
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;

        let mut new_curr = c.clone();
        new_curr.id = CurrencyId::new(last_id);
        Ok(new_curr)
    }

    fn update(&self, c: &Currency) -> Result<(), CurrencyError> {
        let (sql, values) = Query::update()
            .table(Alias::new("CURRENCYFORMATS_V1"))
            .values([
                (Alias::new("CURRENCYNAME"), c.name.clone().into()),
                (Alias::new("PFX_SYMBOL"), c.pfx_symbol.clone().into()),
                (Alias::new("SFX_SYMBOL"), c.sfx_symbol.clone().into()),
                (Alias::new("DECIMAL_POINT"), c.decimal_point.clone().into()),
                (
                    Alias::new("GROUP_SEPARATOR"),
                    c.group_separator.clone().into(),
                ),
                (Alias::new("UNIT_NAME"), c.unit_name.clone().into()),
                (Alias::new("CENT_NAME"), c.cent_name.clone().into()),
                (Alias::new("SCALE"), c.scale.into()),
                (
                    Alias::new("BASECONVRATE"),
                    c.base_conv_rate.v1.clone().into(),
                ),
                (Alias::new("CURRENCY_SYMBOL"), c.symbol.clone().into()),
                (Alias::new("CURRENCY_TYPE"), c.currency_type.clone().into()),
            ])
            .and_where(Expr::col(Alias::new("CURRENCYID")).eq(c.id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn update_partial(
        &self,
        id: CurrencyId,
        update: crate::domain::currencies::CurrencyUpdate,
    ) -> Result<(), CurrencyError> {
        let mut query = Query::update();
        query.table(Alias::new("CURRENCYFORMATS_V1"));

        let mut has_values = false;

        if let Some(name) = update.name {
            query.value(Alias::new("CURRENCYNAME"), SimpleExpr::from(name));
            has_values = true;
        }
        if let Some(pfx) = update.pfx_symbol {
            query.value(Alias::new("PFX_SYMBOL"), SimpleExpr::from(pfx));
            has_values = true;
        }
        if let Some(sfx) = update.sfx_symbol {
            query.value(Alias::new("SFX_SYMBOL"), SimpleExpr::from(sfx));
            has_values = true;
        }
        if let Some(dp) = update.decimal_point {
            query.value(Alias::new("DECIMAL_POINT"), SimpleExpr::from(dp));
            has_values = true;
        }
        if let Some(gs) = update.group_separator {
            query.value(Alias::new("GROUP_SEPARATOR"), SimpleExpr::from(gs));
            has_values = true;
        }
        if let Some(un) = update.unit_name {
            query.value(Alias::new("UNIT_NAME"), SimpleExpr::from(un));
            has_values = true;
        }
        if let Some(cn) = update.cent_name {
            query.value(Alias::new("CENT_NAME"), SimpleExpr::from(cn));
            has_values = true;
        }
        if let Some(scale) = update.scale {
            query.value(Alias::new("SCALE"), SimpleExpr::from(scale));
            has_values = true;
        }
        if let Some(rate) = update.base_conv_rate {
            query.value(Alias::new("BASECONVRATE"), SimpleExpr::from(rate.v1));
            has_values = true;
        }
        if let Some(symbol) = update.symbol {
            query.value(Alias::new("CURRENCY_SYMBOL"), SimpleExpr::from(symbol));
            has_values = true;
        }
        if let Some(ct) = update.currency_type {
            query.value(Alias::new("CURRENCY_TYPE"), SimpleExpr::from(ct));
            has_values = true;
        }

        if !has_values {
            return Ok(());
        }

        query.and_where(Expr::col(Alias::new("CURRENCYID")).eq(id.v1));

        let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn delete(&self, id: CurrencyId) -> Result<(), CurrencyError> {
        let (sql, values) = Query::delete()
            .from_table(Alias::new("CURRENCYFORMATS_V1"))
            .and_where(Expr::col(Alias::new("CURRENCYID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }
}
