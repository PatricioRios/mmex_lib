use crate::api::MmexContext;
use crate::domain::categories::{Category, CategoryError, CategoryId, CategoryUpdate};
use crate::MmexError;
use std::sync::{Arc, Mutex};

/// Gestor especializado en la administración de categorías y subcategorías.
#[derive(uniffi::Object)]
pub struct CategoryManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl CategoryManager {
    /// Obtiene la lista completa de categorías registradas.
    pub fn get_all(&self) -> Result<Vec<Category>, CategoryError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CategoryError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.categories().get_all_categories()?)
    }

    /// Busca una categoría específica por su identificador único.
    pub fn get_by_id(&self, id: i64) -> Result<Option<Category>, CategoryError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CategoryError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx.categories().get_category_by_id(CategoryId { v1: id })?)
    }

    /// Obtiene las subcategorías asociadas a una categoría padre.
    pub fn get_subcategories(&self, parent_id: i64) -> Result<Vec<Category>, CategoryError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CategoryError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx
            .categories()
            .get_subcategories(CategoryId { v1: parent_id })?)
    }

    /// Crea una nueva categoría con el nombre y padre opcional proporcionados.
    pub fn create(&self, name: String, parent_id: Option<i64>) -> Result<Category, CategoryError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CategoryError::Common(MmexError::Internal(e.to_string())))?;
        Ok(ctx
            .categories()
            .create_category(&name, parent_id.map(|v| CategoryId { v1: v }))?)
    }

    /// Actualiza la información de una categoría existente.
    pub fn update(&self, category: Category) -> Result<(), CategoryError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CategoryError::Common(MmexError::Internal(e.to_string())))?;
        ctx.categories().update_category(&category)?;
        Ok(())
    }

    /// Actualiza parcialmente la información de una categoría.
    pub fn update_partial(&self, id: i64, update: CategoryUpdate) -> Result<(), CategoryError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CategoryError::Common(MmexError::Internal(e.to_string())))?;
        ctx.categories()
            .update_category_partial(CategoryId { v1: id }, update)?;
        Ok(())
    }

    /// Elimina una categoría de la base de datos.
    pub fn delete(&self, id: i64) -> Result<(), CategoryError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| CategoryError::Common(MmexError::Internal(e.to_string())))?;
        ctx.categories().delete_category(CategoryId { v1: id })?;
        Ok(())
    }

    /// Obtiene todas las categorías en formato JSON.
    pub fn get_all_json(&self) -> Result<String, CategoryError> {
        let categories = self.get_all()?;
        serde_json::to_string(&categories)
            .map_err(|e| CategoryError::Common(MmexError::Internal(e.to_string())))
    }
}
