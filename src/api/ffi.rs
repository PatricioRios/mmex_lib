use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::path::Path;
use crate::api::MmexContext;

/// Abre un contexto de MMEX. Devuelve un puntero opaco o NULL si falla.
#[no_mangle]
pub extern "C"  fn mmex_context_open(path: *const c_char, key: *const c_char) -> *mut c_void {

    if path.is_null() { return std::ptr::null_mut(); }

    let c_path = unsafe { CStr::from_ptr(path) }.to_string_lossy();
    let c_key = if key.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(key) }.to_string_lossy())
    };

    match MmexContext::open(Path::new(&*c_path), c_key.as_deref()) {
        Ok(ctx) => Box::into_raw(Box::new(ctx)) as *mut c_void,
        Err(_) => std::ptr::null_mut(),
    }
}

/// Cierra y libera el contexto.
#[no_mangle]
pub extern "C" fn mmex_context_free(ctx: *mut c_void) {
    if !ctx.is_null() {
        unsafe {
            let _ = Box::from_raw(ctx as *mut MmexContext);
        }
    }
}

/// Libera una string creada por Rust.
#[no_mangle]
pub extern "C" fn mmex_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

/// Obtiene todas las cuentas en formato JSON.
#[no_mangle]
pub extern "C" fn mmex_get_accounts(ctx: *mut c_void) -> *mut c_char {
    let context = unsafe { &*(ctx as *mut MmexContext) };
    let service = context.accounts();
    
    match service.get_all_accounts() {
        Ok(accounts) => {
            let json = serde_json::to_string(&accounts).unwrap_or_default();
            CString::new(json).unwrap().into_raw()
        },
        Err(e) => {
            let err_json = format!(r#"{{"error": "{}"}}"#, e);
            CString::new(err_json).unwrap().into_raw()
        }
    }
}

/// Obtiene el balance de una cuenta en formato JSON.
#[no_mangle]
pub extern "C" fn mmex_get_account_balance(ctx: *mut c_void, account_id: i64) -> *mut c_char {
    let context = unsafe { &*(ctx as *mut MmexContext) };
    let service = context.accounts();
    
    match service.get_account_balance(crate::domain::types::AccountId(account_id)) {
        Ok(balance) => {
            let json = serde_json::to_string(&balance).unwrap_or_default();
            CString::new(json).unwrap().into_raw()
        },
        Err(e) => {
            let err_json = format!(r#"{{"error": "{}"}}"#, e);
            CString::new(err_json).unwrap().into_raw()
        }
    }
}
