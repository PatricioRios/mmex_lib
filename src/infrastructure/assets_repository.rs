use chrono::NaiveDate;
use rusqlite::Row;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sea_query::{Alias, Expr, Query, SimpleExpr, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use std::str::FromStr;

use crate::domain::assets::{Asset, AssetError, AssetId, AssetRepository, AssetStatus};
use crate::domain::types::{CurrencyId, MmexDate, Money};
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
            id: AssetId::new(row.get("ASSETID")?),
            name: row.get("ASSETNAME")?,
            start_date: MmexDate::from(start_date),
            status: AssetStatus::from(row.get::<_, String>("ASSETSTATUS").unwrap_or_default()),
            currency_id: row
                .get::<_, Option<i64>>("CURRENCYID")?
                .map(CurrencyId::new),
            value_change_mode: row.get("VALUECHANGEMODE")?,
            value: Money::from(value_val),
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
            .from(Alias::new("ASSETS_V1"))
            .build(SqliteQueryBuilder);

        Ok(self
            .executor
            .query_map_ext(&sql, [], |row| AssetMapper::map_row(row))?)
    }

    fn find_by_id(&self, id: AssetId) -> Result<Option<Asset>, AssetError> {
        let (sql, values) = Query::select()
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
            .from(Alias::new("ASSETS_V1"))
            .and_where(Expr::col(Alias::new("ASSETID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        match self
            .executor
            .query_row_ext(&sql, &values.as_params()[..], |row| {
                AssetMapper::map_row(row)
            }) {
            Ok(asset) => Ok(Some(asset)),
            Err(MmexError::NotFound) => Ok(None),
            Err(e) => Err(AssetError::Common(e)),
        }
    }

    fn insert(&self, a: &Asset) -> Result<Asset, AssetError> {
        let (sql, values) = Query::insert()
            .into_table(Alias::new("ASSETS_V1"))
            .columns([
                Alias::new("STARTDATE"),
                Alias::new("ASSETNAME"),
                Alias::new("ASSETSTATUS"),
                Alias::new("CURRENCYID"),
                Alias::new("VALUECHANGEMODE"),
                Alias::new("VALUE"),
                Alias::new("VALUECHANGE"),
                Alias::new("NOTES"),
                Alias::new("VALUECHANGERATE"),
                Alias::new("ASSETTYPE"),
            ])
            .values_panic([
                a.start_date.v1.clone().into(),
                a.name.clone().into(),
                a.status.to_string().into(),
                a.currency_id.map(|id| id.v1).into(),
                a.value_change_mode.clone().into(),
                a.value.v1.clone().into(),
                a.value_change.clone().into(),
                a.notes.clone().into(),
                a.value_change_rate.into(),
                a.asset_type.clone().into(),
            ])
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;

        let last_id: i64 = self
            .executor
            .query_row_ext("SELECT last_insert_rowid()", [], |r| r.get(0))?;

        let mut new_asset = a.clone();
        new_asset.id = AssetId::new(last_id);
        Ok(new_asset)
    }

    fn update(&self, a: &Asset) -> Result<(), AssetError> {
        let (sql, values) = Query::update()
            .table(Alias::new("ASSETS_V1"))
            .values([
                (Alias::new("STARTDATE"), a.start_date.v1.clone().into()),
                (Alias::new("ASSETNAME"), a.name.clone().into()),
                (Alias::new("ASSETSTATUS"), a.status.to_string().into()),
                (
                    Alias::new("CURRENCYID"),
                    a.currency_id.map(|id| id.v1).into(),
                ),
                (
                    Alias::new("VALUECHANGEMODE"),
                    a.value_change_mode.clone().into(),
                ),
                (Alias::new("VALUE"), a.value.v1.clone().into()),
                (Alias::new("VALUECHANGE"), a.value_change.clone().into()),
                (Alias::new("NOTES"), a.notes.clone().into()),
                (Alias::new("VALUECHANGERATE"), a.value_change_rate.into()),
                (Alias::new("ASSETTYPE"), a.asset_type.clone().into()),
            ])
            .and_where(Expr::col(Alias::new("ASSETID")).eq(a.id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn update_partial(
        &self,
        id: AssetId,
        update: crate::domain::assets::AssetUpdate,
    ) -> Result<(), AssetError> {
        let mut query = Query::update();
        query.table(Alias::new("ASSETS_V1"));

        let mut has_values = false;

        if let Some(name) = update.name {
            query.value(Alias::new("ASSETNAME"), SimpleExpr::from(name));
            has_values = true;
        }
        if let Some(sd) = update.start_date {
            query.value(Alias::new("STARTDATE"), SimpleExpr::from(sd.v1));
            has_values = true;
        }
        if let Some(status) = update.status {
            query.value(
                Alias::new("ASSETSTATUS"),
                SimpleExpr::from(status.to_string()),
            );
            has_values = true;
        }
        if let Some(curr_id) = update.currency_id {
            query.value(Alias::new("CURRENCYID"), SimpleExpr::from(curr_id.v1));
            has_values = true;
        }
        if let Some(vcm) = update.value_change_mode {
            query.value(Alias::new("VALUECHANGEMODE"), SimpleExpr::from(vcm));
            has_values = true;
        }
        if let Some(val) = update.value {
            query.value(Alias::new("VALUE"), SimpleExpr::from(val.v1));
            has_values = true;
        }
        if let Some(vc) = update.value_change {
            query.value(Alias::new("VALUECHANGE"), SimpleExpr::from(vc));
            has_values = true;
        }
        if let Some(notes) = update.notes {
            query.value(Alias::new("NOTES"), SimpleExpr::from(notes));
            has_values = true;
        }
        if let Some(vcr) = update.value_change_rate {
            query.value(Alias::new("VALUECHANGERATE"), SimpleExpr::from(vcr));
            has_values = true;
        }
        if let Some(at) = update.asset_type {
            query.value(Alias::new("ASSETTYPE"), SimpleExpr::from(at));
            has_values = true;
        }

        if !has_values {
            return Ok(());
        }

        query.and_where(Expr::col(Alias::new("ASSETID")).eq(id.v1));

        let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }

    fn delete(&self, id: AssetId) -> Result<(), AssetError> {
        let (sql, values) = Query::delete()
            .from_table(Alias::new("ASSETS_V1"))
            .and_where(Expr::col(Alias::new("ASSETID")).eq(id.v1))
            .build_rusqlite(SqliteQueryBuilder);

        self.executor.execute_ext(&sql, &values.as_params()[..])?;
        Ok(())
    }
}
