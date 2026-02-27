use std::sync::{Arc, Mutex};
use crate::api::MmexContext;
use crate::error::MmexError;
use crate::domain::types::AccountId;

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct MmexEngine {
    context: Arc<Mutex<MmexContext>>,
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl MmexEngine {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub fn new(path: String, key: Option<String>) -> Result<Arc<Self>, MmexError> {
        let ctx = MmexContext::open((&path).as_ref(), key)?;
        Ok(Arc::new(Self {
            context: Arc::new(Mutex::new(ctx)),
        }))
    }

    pub fn get_db_version(&self) -> Result<String, MmexError> {
        let ctx = self.context.lock().map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.support().get_db_version()
    }

    pub fn get_accounts_json(&self) -> Result<String, MmexError> {
        let ctx = self.context.lock().map_err(|e| MmexError::Internal(e.to_string()))?;
        let accounts = ctx.accounts().get_all_accounts()?;
        serde_json::to_string(&accounts).map_err(|e| MmexError::Internal(e.to_string()))
    }

    pub fn get_account_balance_json(&self, account_id: i64) -> Result<String, MmexError> {
        let ctx = self.context.lock().map_err(|e| MmexError::Internal(e.to_string()))?;
        let balance = ctx.accounts().get_account_balance(AccountId(account_id))?;
        serde_json::to_string(&balance).map_err(|e| MmexError::Internal(e.to_string()))
    }

    pub fn get_transactions_json(&self) -> Result<String, MmexError> {
        let ctx = self.context.lock().map_err(|e| MmexError::Internal(e.to_string()))?;
        let txs = ctx.transactions().get_all_transactions()?;
        serde_json::to_string(&txs).map_err(|e| MmexError::Internal(e.to_string()))
    }
}

// Mantenemos los extern "C" para compatibilidad con lenguajes que no usen UniFFI
#[no_mangle]
pub extern "C" fn mmex_engine_new(path: *const std::os::raw::c_char, key: *const std::os::raw::c_char) -> *mut MmexEngine {
    use std::ffi::CStr;
    if path.is_null() { return std::ptr::null_mut(); }
    let c_path = unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned();
    let c_key = if key.is_null() { None } else { Some(unsafe { CStr::from_ptr(key) }.to_string_lossy().into_owned()) };
    
    match MmexEngine::new(c_path, c_key) {
        Ok(engine) => Arc::into_raw(engine) as *mut MmexEngine,
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mmex_engine_free(engine: *mut MmexEngine) {
    if !engine.is_null() {
        unsafe {
            let _ = Arc::from_raw(engine);
        }
    }
}
