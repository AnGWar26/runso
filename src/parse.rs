use std::ffi::{CString, c_int, c_uint};
use std::os::raw::c_char;

//pub fn parse_to_ctypes(input: Vec<String>) -> Vec<_> {}

pub fn str_to_cstring(input: &str) -> CString {
    CString::new(input).unwrap()
}

pub fn str_to_c_int(input: &str) -> c_int {
    input.parse::<i32>().unwrap()
}

pub fn str_to_c_uint(input: &str) -> c_uint {
    input.parse::<u32>().unwrap()
}
