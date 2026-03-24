use crate::domain::stocks::{Stock, StockError, StockId, StockRepository};
use crate::infrastructure::stocks_repository::SqlStockRepository;
use rusqlite::Connection;

pub struct StockService<'a> {
    conn: &'a Connection,
}

impl<'a> StockService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_stocks(&self) -> Result<Vec<Stock>, StockError> {
        let repo = SqlStockRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_stock_by_id(&self, id: StockId) -> Result<Option<Stock>, StockError> {
        let repo = SqlStockRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_stock(&self, stock: &Stock) -> Result<Stock, StockError> {
        let repo = SqlStockRepository::new(self.conn);
        repo.insert(stock)
    }

    pub fn update_stock(&self, stock: &Stock) -> Result<(), StockError> {
        let repo = SqlStockRepository::new(self.conn);
        repo.update(stock)
    }

    pub fn update_stock_partial(
        &self,
        id: StockId,
        update: crate::domain::stocks::StockUpdate,
    ) -> Result<(), StockError> {
        let repo = SqlStockRepository::new(self.conn);
        repo.update_partial(id, update)
    }

    pub fn delete_stock(&self, id: StockId) -> Result<(), StockError> {
        let repo = SqlStockRepository::new(self.conn);
        repo.delete(id)
    }
}
