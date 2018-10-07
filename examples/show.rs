#![feature(proc_macro_hygiene)]

extern crate literal_cstr;

use std::os::raw::c_char;
use literal_cstr::c;

extern {
    fn printf(format: *const c_char, ...);
}

fn main() {
    let s = c!("The answer is %d.\n");

    unsafe {
        printf(s.as_ptr(), 42);
    }
}
