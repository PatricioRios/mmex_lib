use rusqlite::{Connection, Transaction, Params, Row};
use crate::error::MmexError;

/// Abstracción para ejecutar comandos SQL.
/// No usamos dyn porque los métodos son genéricos.
pub trait DbExecutor {
    fn query_row_ext<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T, MmexError>
    where
        P: Params,
        F: FnOnce(&Row<'_>) -> rusqlite::Result<T>;

    fn execute_ext<P>(&self, sql: &str, params: P) -> Result<usize, MmexError>
    where
        P: Params;

    fn query_map_ext<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<Vec<T>, MmexError>
    where
        P: Params,
        F: FnMut(&Row<'_>) -> rusqlite::Result<T>;
}

impl DbExecutor for Connection {
    fn query_row_ext<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T, MmexError>
    where
        P: Params,
        F: FnOnce(&Row<'_>) -> rusqlite::Result<T>,
    {
        self.query_row(sql, params, f).map_err(MmexError::from)
    }

    fn execute_ext<P>(&self, sql: &str, params: P) -> Result<usize, MmexError>
    where
        P: Params,
    {
        self.execute(sql, params).map_err(MmexError::from)
    }

    fn query_map_ext<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<Vec<T>, MmexError>
    where
        P: Params,
        F: FnMut(&Row<'_>) -> rusqlite::Result<T>,
    {
        let mut stmt = self.prepare(sql)?;
        let iter = stmt.query_map(params, f)?;
        let mut result = Vec::new();
        for item in iter {
            result.push(item?);
        }
        Ok(result)
    }
}

impl<'a> DbExecutor for Transaction<'a> {
    fn query_row_ext<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T, MmexError>
    where
        P: Params,
        F: FnOnce(&Row<'_>) -> rusqlite::Result<T>,
    {
        self.query_row(sql, params, f).map_err(MmexError::from)
    }

    fn execute_ext<P>(&self, sql: &str, params: P) -> Result<usize, MmexError>
    where
        P: Params,
    {
        self.execute(sql, params).map_err(MmexError::from)
    }

    fn query_map_ext<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<Vec<T>, MmexError>
    where
        P: Params,
        F: FnMut(&Row<'_>) -> rusqlite::Result<T>,
    {
        let mut stmt = self.prepare(sql)?;
        let iter = stmt.query_map(params, f)?;
        let mut result = Vec::new();
        for item in iter {
            result.push(item?);
        }
        Ok(result)
    }
}
