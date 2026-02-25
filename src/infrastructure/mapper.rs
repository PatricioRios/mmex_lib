use rusqlite::Row;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::domain::models::Account;
use crate::domain::types::{AccountId, Money};

pub struct AccountMapper;

impl AccountMapper {
    pub fn map_row(row: &Row) -> rusqlite::Result<Account> {
        let id: i32 = row.get("ACCOUNTID")?;
        let name: String = row.get("ACCOUNTNAME")?;
        let initial_bal_str: String = row.get("INITIALBAL")?; // MMEX a veces guarda decimales como string
        
        let initial_balance = Decimal::from_str(&initial_bal_str)
            .unwrap_or_else(|_| Decimal::ZERO);

        Ok(Account {
            id: AccountId(id),
            name,
            initial_balance: Money(initial_balance),
            account_type: row.get("ACCOUNTTYPE")?,
            status: row.get("ACCOUNTSTATUS")?,
        })
    }
}
