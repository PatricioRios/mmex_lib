use crate::api::MmexContext;
use crate::ffi::accounts::AccountManager;
use crate::ffi::assets::AssetManager;
use crate::ffi::categories::CategoryManager;
use crate::ffi::currencies::CurrencyManager;
use crate::ffi::payees::PayeeManager;
use crate::ffi::scheduled::ScheduledManager;
use crate::ffi::stocks::StockManager;
use crate::ffi::support::SupportManager;
use crate::ffi::tags::TagManager;
use crate::ffi::transactions::TransactionManager;
use crate::MmexError;
use std::sync::{Arc, Mutex};

/// Motor principal de mmex_lib expuesto para otros lenguajes (Python, Kotlin, Swift).
/// Proporciona acceso a todos los gestores de la base de datos de Money Manager EX.
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

    /// Accede al gestor de beneficiarios (Payees).
    pub fn payees(&self) -> Arc<PayeeManager> {
        Arc::new(PayeeManager {
            context: self.context.clone(),
        })
    }

    /// Accede al gestor de monedas (Currencies).
    pub fn currencies(&self) -> Arc<CurrencyManager> {
        Arc::new(CurrencyManager {
            context: self.context.clone(),
        })
    }

    /// Accede al gestor de categorías (Categories).
    pub fn categories(&self) -> Arc<CategoryManager> {
        Arc::new(CategoryManager {
            context: self.context.clone(),
        })
    }

    /// Accede al gestor de transacciones (Transactions).
    pub fn transactions(&self) -> Arc<TransactionManager> {
        Arc::new(TransactionManager {
            context: self.context.clone(),
        })
    }

    /// Accede al gestor de transacciones programadas (Scheduled).
    pub fn scheduled(&self) -> Arc<ScheduledManager> {
        Arc::new(ScheduledManager {
            context: self.context.clone(),
        })
    }

    /// Accede al gestor de activos (Assets).
    pub fn assets(&self) -> Arc<AssetManager> {
        Arc::new(AssetManager {
            context: self.context.clone(),
        })
    }

    /// Accede al gestor de acciones y valores (Stocks).
    pub fn stocks(&self) -> Arc<StockManager> {
        Arc::new(StockManager {
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
