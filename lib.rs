//! lib.rs --- dysk library
use std::ffi::{c_char, c_int, CString};
#[no_mangle]
pub extern "C" fn hello() -> *const c_char {
  CString::new("hello from rust").unwrap().into_raw()}
#[no_mangle]
pub extern "C" fn plus(a:c_int,b:c_int) -> c_int {a+b}
#[no_mangle]
pub extern "C" fn plus1(n:c_int) -> c_int {n+1}
