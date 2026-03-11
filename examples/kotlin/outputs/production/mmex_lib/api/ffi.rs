use crate::api::MmexContext;
use crate::domain::tags::Tag;
use crate::domain::types::{AccountId, TagId};
use crate::MmexError;
use std::sync::{Arc, Mutex};

#[derive(uniffi::Object)]
pub struct MmexEngine {
    context: Arc<Mutex<MmexContext>>,
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

    // --- Tags CRUD ---

    pub fn get_tags(&self) -> Result<Vec<Tag>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.tags().get_all_tags()?)
    }

    pub fn get_tag_by_id(&self, id: i64) -> Result<Option<Tag>, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.tags().get_tag_by_id(TagId { v1: id })?)
    }

    pub fn create_tag(&self, name: String) -> Result<Tag, MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        Ok(ctx.tags().create_tag(&name)?)
    }

    pub fn update_tag(&self, id: i64, name: String) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        let tag = Tag {
            id: TagId { v1: id },
            name,
        };
        ctx.tags().update_tag(&tag)?;
        Ok(())
    }

    pub fn delete_tag(&self, id: i64) -> Result<(), MmexError> {
        let ctx = self
            .context
            .lock()
            .map_err(|e| MmexError::Internal(e.to_string()))?;
        ctx.tags().delete_tag(TagId { v1: id })?;
        Ok(())
    }

    // Mantener versiones JSON por compatibilidad si es necesario, o eliminarlas.
    // El usuario pidió "estructuras de datos relacionadas", así que las nuevas son preferibles.

    pub fn get_tags_json(&self) -> Result<String, MmexError> {
        let tags = self.get_tags()?;
        serde_json::to_string(&tags).map_err(|e| MmexError::Internal(e.to_string()))
    }

    pub fn get_tag_by_id_json(&self, id: i64) -> Result<String, MmexError> {
        let tag = self.get_tag_by_id(id)?;
        serde_json::to_string(&tag).map_err(|e| MmexError::Internal(e.to_string()))
    }

    pub fn create_tag_json(&self, name: String) -> Result<String, MmexError> {
        let tag = self.create_tag(name)?;
        serde_json::to_string(&tag).map_err(|e| MmexError::Internal(e.to_string()))
    }
}

// --- Capa Raw C-ABI (Para Go, C, Bun FFI, etc.) ---

#[no_mangle]
pub extern "C" fn mmex_engine_new(
    path: *const std::os::raw::c_char,
    key: *const std::os::raw::c_char,
) -> *mut Arc<MmexEngine> {
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
pub extern "C" fn mmex_engine_free(engine: *mut Arc<MmexEngine>) {
    if !engine.is_null() {
        unsafe {
            let _ = Box::from_raw(engine);
        }
    }
}

#[no_mangle]
pub extern "C" fn mmex_get_accounts_json(
    engine: *mut Arc<MmexEngine>,
) -> *mut std::os::raw::c_char {
    let engine = unsafe { &*(engine) };
    match engine.get_accounts_json() {
        Ok(json) => std::ffi::CString::new(json).unwrap().into_raw(),
        Err(e) => std::ffi::CString::new(format!(r#"{{"error": "{}"}}"#, e))
            .unwrap()
            .into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn mmex_get_tags_json(engine: *mut Arc<MmexEngine>) -> *mut std::os::raw::c_char {
    let engine = unsafe { &*(engine) };
    match engine.get_tags_json() {
        Ok(json) => std::ffi::CString::new(json).unwrap().into_raw(),
        Err(e) => std::ffi::CString::new(format!(r#"{{"error": "{}"}}"#, e))
            .unwrap()
            .into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn mmex_create_tag_json(
    engine: *mut Arc<MmexEngine>,
    name: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    use std::ffi::CStr;
    let engine = unsafe { &*(engine) };
    let c_name = unsafe { CStr::from_ptr(name) }
        .to_string_lossy()
        .into_owned();

    match engine.create_tag_json(c_name) {
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
