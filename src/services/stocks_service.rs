use rusqlite::Connection;
use crate::domain::stocks::{Stock, StockId, StockRepository};
use crate::infrastructure::stocks_repository::SqlStockRepository;
use crate::error::MmexError;

pub struct StockService<'a> {
    conn: &'a Connection,
}

impl<'a> StockService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_stocks(&self) -> Result<Vec<Stock>, MmexError> {
        let repo = SqlStockRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_stock_by_id(&self, id: StockId) -> Result<Option<Stock>, MmexError> {
        let repo = SqlStockRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_stock(&self, stock: &Stock) -> Result<Stock, MmexError> {
        let repo = SqlStockRepository::new(self.conn);
        repo.insert(stock)
    }

    pub fn update_stock(&self, stock: &Stock) -> Result<(), MmexError> {
        let repo = SqlStockRepository::new(self.conn);
        repo.update(stock)
    }

    pub fn delete_stock(&self, id: StockId) -> Result<(), MmexError> {
        let repo = SqlStockRepository::new(self.conn);
        repo.delete(id)
    }
}
