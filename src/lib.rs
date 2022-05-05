extern crate libc;

use core::slice;
use libc::{c_char, c_uchar};
use rlua::Lua;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn char_cnt(s: *const c_char) -> usize {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();
    r_str.chars().count()
}

#[no_mangle]
pub extern "C" fn free_str(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern "C" fn concat(s1: *mut c_char, s2: *mut c_char) -> *mut c_char {
    let (c_str1, c_str2) = unsafe {
        assert!(!s1.is_null());
        assert!(!s2.is_null());
        (CStr::from_ptr(s1), CStr::from_ptr(s2))
    };

    let mut _ret = c_str1.to_str().expect("invalid utf-8").to_owned();
    _ret.push_str(c_str2.to_str().expect("invalid utf-8"));
    CString::new(_ret).expect("nul error").into_raw()
}

#[no_mangle]
pub extern "C" fn eval(s: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };

    let r_string = c_str.to_str().expect("invalid utf-8");

    let ret = Lua::new().context(|lua| match lua.load(r_string).eval::<String>() {
        Ok(value) => value,
        Err(err) => format!("{}", err),
    });
    CString::new(ret).expect("nul error").into_raw()
}

#[no_mangle]
pub extern "C" fn deliver(data: *const c_uchar, len: usize) -> *mut c_char {
    let bytes = unsafe { slice::from_raw_parts(data, len) };
    let bytes: Vec<u8> = Vec::from(bytes);
    CString::new(String::from_utf8(bytes).expect("invalid utf-8"))
        .expect("nul error")
        .into_raw()
}
