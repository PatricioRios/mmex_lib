use rusqlite::Connection;
use crate::domain::assets::{Asset, AssetId, AssetRepository};
use crate::infrastructure::assets_repository::SqlAssetRepository;
use crate::error::MmexError;

pub struct AssetService<'a> {
    conn: &'a Connection,
}

impl<'a> AssetService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_assets(&self) -> Result<Vec<Asset>, MmexError> {
        let repo = SqlAssetRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_asset_by_id(&self, id: AssetId) -> Result<Option<Asset>, MmexError> {
        let repo = SqlAssetRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_asset(&self, asset: &Asset) -> Result<Asset, MmexError> {
        let repo = SqlAssetRepository::new(self.conn);
        repo.insert(asset)
    }

    pub fn update_asset(&self, asset: &Asset) -> Result<(), MmexError> {
        let repo = SqlAssetRepository::new(self.conn);
        repo.update(asset)
    }

    pub fn delete_asset(&self, id: AssetId) -> Result<(), MmexError> {
        let repo = SqlAssetRepository::new(self.conn);
        repo.delete(id)
    }
}
