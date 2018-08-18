#![feature(proc_macro_non_items)]

extern crate literal_cstr;

use std::ffi::CString;
use literal_cstr::c;

#[test]
fn test_literal() {
    let s = c!("Hello, world!");

    assert_eq!(CString::new("Hello, world!").unwrap(), s);
}

#[test]
#[should_panic(expected="NulError")]
fn test_nul_byte() {
    let _ = c!("Hello, \0world!");
}
