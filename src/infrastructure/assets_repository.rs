use chrono::NaiveDate;
use rusqlite::Row;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use std::str::FromStr;

use crate::domain::assets::{Asset, AssetError, AssetId, AssetRepository, AssetStatus};
use crate::domain::types::{CurrencyId, Money};
use crate::infrastructure::db_executor::DbExecutor;
use crate::MmexError;

pub struct AssetMapper;

impl AssetMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Asset> {
        let value_val = if let Ok(val) = row.get::<_, f64>("VALUE") {
            Decimal::from_f64(val).unwrap_or(Decimal::ZERO)
        } else if let Ok(s) = row.get::<_, String>("VALUE") {
            Decimal::from_str(&s).unwrap_or(Decimal::ZERO)
        } else {
            Decimal::ZERO
        };

        let date_str: String = row.get("STARTDATE")?;
        let start_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

        Ok(Asset {
            id: AssetId {
                v1: row.get("ASSETID")?,
            },
            name: row.get("ASSETNAME")?,
            start_date,
            status: AssetStatus::from(row.get::<_, String>("ASSETSTATUS").unwrap_or_default()),
            currency_id: row
                .get::<_, Option<i64>>("CURRENCYID")?
                .map(|v1| CurrencyId { v1 }),
            value_change_mode: row.get("VALUECHANGEMODE")?,
            value: Money(value_val),
            value_change: row.get("VALUECHANGE")?,
            notes: row.get("NOTES")?,
            value_change_rate: row.get("VALUECHANGERATE")?,
            asset_type: row.get("ASSETTYPE")?,
        })
    }
}

pub struct SqlAssetRepository<'a, E: DbExecutor> {
    executor: &'a E,
}

impl<'a, E: DbExecutor> SqlAssetRepository<'a, E> {
    pub fn new(executor: &'a E) -> Self {
        Self { executor }
    }
}

impl<'a, E: DbExecutor> AssetRepository for SqlAssetRepository<'a, E> {
    fn find_all(&self) -> Result<Vec<Asset>, AssetError> {
        let (sql, _) = Query::select()
            .columns([
                "ASSETID",
                "STARTDATE",
                "ASSETNAME",
                "ASSETSTATUS",
                "CURRENCYID",
                "VALUECHANGEMODE",
                "VALUE",
                "VALUECHANGE",
                "NOTES",
                "VALUECHANGERATE",
                "ASSETTYPE",
            ])
            .from_as("ASSETS_V1", "a")
            .build(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| AssetMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: AssetId) -> Result<Option<Asset>, AssetError> {
        let (sql, _) = Query::select()
            .columns([
                "ASSETID",
                "STARTDATE",
                "ASSETNAME",
                "ASSETSTATUS",
                "CURRENCYID",
                "VALUECHANGEMODE",
                "VALUE",
                "VALUECHANGE",
                "NOTES",
                "VALUECHANGERATE",
                "ASSETTYPE",
            ])
            .from_as("ASSETS_V1", "a")
            .and_where(Expr::col("ASSETID").eq(id.v1))
            .build(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, [id.v1], |row| AssetMapper::map_row(row))
        {
            Ok(asset) => Ok(Some(asset)),
            Err(MmexError::Database(e)) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(AssetError::Common(e)),
        }
    }

    fn insert(&self, a: &Asset) -> Result<Asset, AssetError> {
        let sql = "INSERT INTO ASSETS_V1 (STARTDATE, ASSETNAME, ASSETSTATUS, CURRENCYID, VALUECHANGEMODE, VALUE, VALUECHANGE, NOTES, VALUECHANGERATE, ASSETTYPE) 
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        self.executor.execute_ext(
            sql,
            (
                a.start_date.to_string(),
                &a.name,
                a.status.to_string(),
                a.currency_id.map(|id| id.v1),
                &a.value_change_mode,
                a.value.0.to_string(),
                &a.value_change,
                &a.notes,
                a.value_change_rate,
                &a.asset_type,
            ),
        )?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;
        let mut new_asset = a.clone();
        new_asset.id = AssetId { v1: last_id };
        Ok(new_asset)
    }

    fn update(&self, a: &Asset) -> Result<(), AssetError> {
        let sql = "UPDATE ASSETS_V1 SET 
                   STARTDATE = ?, ASSETNAME = ?, ASSETSTATUS = ?, CURRENCYID = ?, VALUECHANGEMODE = ?, VALUE = ?, VALUECHANGE = ?, NOTES = ?, VALUECHANGERATE = ?, ASSETTYPE = ?
                   WHERE ASSETID = ?";

        self.executor.execute_ext(
            sql,
            (
                a.start_date.to_string(),
                &a.name,
                a.status.to_string(),
                a.currency_id.map(|id| id.v1),
                &a.value_change_mode,
                a.value.0.to_string(),
                &a.value_change,
                &a.notes,
                a.value_change_rate,
                &a.asset_type,
                a.id.v1,
            ),
        )?;
        Ok(())
    }

    fn delete(&self, id: AssetId) -> Result<(), AssetError> {
        self.executor
            .execute_ext("DELETE FROM ASSETS_V1 WHERE ASSETID = ?", [id.v1])?;
        Ok(())
    }
}
