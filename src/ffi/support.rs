use crate::api::MmexContext;
use crate::MmexError;
use std::sync::{Arc, Mutex};

#[derive(uniffi::Object)]
pub struct SupportManager {
    pub(crate) context: Arc<Mutex<MmexContext>>,
}

#[uniffi::export]
impl SupportManager {
    pub fn get_db_version(&self) -> Result<String, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.support().get_db_version()?)
    }
}
