use crate::api::MmexContext;
use crate::ffi::accounts::AccountManager;
use crate::ffi::support::SupportManager;
use crate::ffi::tags::TagManager;
use crate::MmexError;
use std::sync::{Arc, Mutex};

#[derive(uniffi::Object)]
pub struct MmexEngine {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl MmexEngine {
    /// Crea una nueva instancia del motor de Money Manager EX.
    ///
    /// # Parámetros
    /// * `path` - Ruta absoluta al archivo de base de datos (.mmb).
    /// * `key` - Clave opcional para bases de datos cifradas (SQLCipher).
    #[uniffi::constructor]
    pub fn new(path: String, key: Option<String>) -> Result<Arc<Self>, MmexError> {
        let ctx = MmexContext::open((&path).as_ref(), key)?;
        Ok(Arc::new(Self {
            context: Arc::new(Mutex::new(ctx)),
        }))
    }

    /// Accede al gestor de etiquetas (Tags).
    pub fn tags(&self) -> Arc<TagManager> {
        Arc::new(TagManager {
            context: self.context.clone(),
        })
    }

    /// Accede al gestor de cuentas (Accounts).
    pub fn accounts(&self) -> Arc<AccountManager> {
        Arc::new(AccountManager {
            context: self.context.clone(),
        })
    }

    /// Accede a utilidades de soporte y metadatos de la base de datos.
    pub fn support(&self) -> Arc<SupportManager> {
        Arc::new(SupportManager {
            context: self.context.clone(),
        })
    }
}
