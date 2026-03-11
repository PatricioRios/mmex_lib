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
    #[uniffi::constructor]
    pub fn new(path: String, key: Option<String>) -> Result<Arc<Self>, MmexError> {
        let ctx = MmexContext::open((&path).as_ref(), key)?;
        Ok(Arc::new(Self {
            context: Arc::new(Mutex::new(ctx)),
        }))
    }

    pub fn tags(&self) -> Arc<TagManager> {
        Arc::new(TagManager {
            context: self.context.clone(),
        })
    }

    pub fn accounts(&self) -> Arc<AccountManager> {
        Arc::new(AccountManager {
            context: self.context.clone(),
        })
    }

    pub fn support(&self) -> Arc<SupportManager> {
        Arc::new(SupportManager {
            context: self.context.clone(),
        })
    }
}
