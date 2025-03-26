use std::ffi::{CString, c_int, c_uint};
use std::os::raw::c_char;

pub fn parse_to_ctypes(input: Vec<&str>, types: Vec<&str>) -> () {}

fn str_to_cstring(input: &str) -> CString {
    CString::new(input).unwrap()
}

fn str_to_c_int(input: &str) -> c_int {
    input.parse::<i32>().unwrap()
}

fn str_to_c_uint(input: &str) -> c_uint {
    input.parse::<u32>().unwrap()
}
