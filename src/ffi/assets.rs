use crate::api::MmexContext;
use crate::domain::assets::{Asset, AssetId};
use crate::MmexError;
use std::sync::{Arc, Mutex};

/// Gestor especializado en la administración de activos (Assets).
#[derive(uniffi::Object)]
pub struct AssetManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl AssetManager {
    /// Obtiene la lista completa de activos registrados.
    pub fn get_all(&self) -> Result<Vec<Asset>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.assets().get_all_assets()?)
    }

    /// Busca un activo específico por su identificador único.
    pub fn get_by_id(&self, id: i64) -> Result<Option<Asset>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.assets().get_asset_by_id(AssetId { v1: id })?)
    }

    /// Crea un nuevo activo en la base de datos.
    pub fn create(&self, asset: Asset) -> Result<Asset, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.assets().create_asset(&asset)?)
    }

    /// Actualiza la información de un activo existente.
    pub fn update(&self, asset: Asset) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.assets().update_asset(&asset)?;
        Ok(())
    }

    /// Elimina un activo de la base de datos.
    pub fn delete(&self, id: i64) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.assets().delete_asset(AssetId { v1: id })?;
        Ok(())
    }

    /// Obtiene todos los activos en formato JSON.
    pub fn get_all_json(&self) -> Result<String, MmexError> {
        let assets = self.get_all()?;
        serde_json::to_string(&assets).map_err(|e| MmexError::Internal(e.to_string()))
    }
}
