use rusqlite::Row;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::domain::accounts::{Account, AccountStatus, AccountType};
use crate::domain::types::{AccountId, CurrencyId, Money};

pub struct AccountMapper;

impl AccountMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Account> {
        let initial_bal = if let Ok(val) = row.get::<_, f64>("INITIALBAL") {
            Decimal::from_f64(val).unwrap_or(Decimal::ZERO)
        } else if let Ok(s) = row.get::<_, String>("INITIALBAL") {
            Decimal::from_str(&s).unwrap_or(Decimal::ZERO)
        } else {
            Decimal::ZERO
        };

        let type_str: String = row.get("ACCOUNTTYPE")?;
        let status_str: String = row.get("STATUS")?;
        let favorite_str: String = row.get("FAVORITEACCT")?;

        Ok(Account {
            id: AccountId {
                v1: row.get("ACCOUNTID")?,
            },
            name: row.get("ACCOUNTNAME")?,
            account_type: AccountType::from(type_str),
            account_num: row.get("ACCOUNTNUM")?,
            status: AccountStatus::from(status_str),
            notes: row.get("NOTES")?,
            initial_balance: Money::from(initial_bal),
            currency_id: CurrencyId {
                v1: row.get("CURRENCYID")?,
            },
            favorite: favorite_str == "TRUE" || favorite_str == "1",
        })
    }
}
