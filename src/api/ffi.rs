use crate::api::MmexContext;
use crate::domain::types::AccountId;
use crate::MmexError;
use std::sync::{Arc, Mutex};

pub struct MmexEngine {
    context: Arc<Mutex<MmexContext>>,
}

impl MmexEngine {
    pub fn new(path: String, key: Option<String>) -> Result<Self, MmexError> {
        let ctx = MmexContext::open((&path).as_ref(), key)?;
        Ok(Self {
            context: Arc::new(Mutex::new(ctx)),
        })
    }

    pub fn get_db_version(&self) -> Result<String, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.support().get_db_version()?)
    }

    pub fn get_accounts_json(&self) -> Result<String, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        let accounts = ctx.accounts().get_all_accounts()?;
        serde_json::to_string(&accounts).map_err(|e| MmexError::Internal(e.to_string()))
    }

    pub fn get_account_balance_json(&self, account_id: i64) -> Result<String, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        let balance = ctx.accounts().get_account_balance(AccountId(account_id))?;
        serde_json::to_string(&balance).map_err(|e| MmexError::Internal(e.to_string()))
    }
}

// --- Capa Raw C-ABI (Para Go, C, Bun FFI, etc.) ---

#[no_mangle]
pub extern "C" fn mmex_engine_new(
    path: *const std::os::raw::c_char,
    key: *const std::os::raw::c_char,
) -> *mut MmexEngine {
    use std::ffi::CStr;
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let c_path = unsafe { CStr::from_ptr(path) }
        .to_string_lossy()
        .into_owned();
    let c_key = if key.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(key) }
                .to_string_lossy()
                .into_owned(),
        )
    };

    match MmexEngine::new(c_path, c_key) {
        Ok(engine) => Box::into_raw(Box::new(engine)),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mmex_engine_free(engine: *mut MmexEngine) {
    if !engine.is_null() {
        unsafe {
            let _ = Box::from_raw(engine);
        }
    }
}

#[no_mangle]
pub extern "C" fn mmex_get_accounts_json(engine: *mut MmexEngine) -> *mut std::os::raw::c_char {
    let engine = unsafe { &*(engine) };
    match engine.get_accounts_json() {
        Ok(json) => std::ffi::CString::new(json).unwrap().into_raw(),
        Err(e) => std::ffi::CString::new(format!(r#"{{"error": "{}"}}"#, e))
            .unwrap()
            .into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn mmex_free_string(s: *mut std::os::raw::c_char) {
    if !s.is_null() {
        unsafe {
            let _ = std::ffi::CString::from_raw(s);
        }
    }
}
