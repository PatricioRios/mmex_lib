use crate::ffi::engine::MmexEngine;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Arc;

#[no_mangle]
pub extern "C" fn mmex_engine_new(path: *const c_char, key: *const c_char) -> *mut Arc<MmexEngine> {
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
pub extern "C" fn mmex_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

#[no_mangle]
pub extern "C" fn mmex_get_accounts_json(engine: *mut Arc<MmexEngine>) -> *mut c_char {
    let engine = unsafe { &*(engine) };
    match engine.accounts().get_all_json() {
        Ok(json) => CString::new(json).unwrap().into_raw(),
        Err(e) => CString::new(format!(r#"{{"error": "{}"}}"#, e))
            .unwrap()
            .into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn mmex_get_tags_json(engine: *mut Arc<MmexEngine>) -> *mut c_char {
    let engine = unsafe { &*(engine) };
    match engine.tags().get_all_json() {
        Ok(json) => CString::new(json).unwrap().into_raw(),
        Err(e) => CString::new(format!(r#"{{"error": "{}"}}"#, e))
            .unwrap()
            .into_raw(),
    }
}
