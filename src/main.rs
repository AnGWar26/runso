use core::str;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod parse;

fn main() {
    // let args: Vec<String> = std::env::args().collect();

    //TODO: error handle these
    let ptr = open_lib("/usr/lib64/libc.so.6").unwrap();
    let fn_handle = find_symbol(ptr, "strlen").unwrap();

    call_func(fn_handle, "VICTORY");

    if close_lib(ptr) == 0 {
        println!("Library closed successfully")
    } else {
        println!("Something went wrong closing the library")
    }
}

/// Given a path to a .so, will retrieve the openhandle to it.
/// Essentially a wrapper around dlopen()
fn open_lib(path: &str) -> Result<*mut libc::c_void, &str> {
    unsafe {
        let c_str = CString::new(path).unwrap();
        let c_chars: *const c_char = c_str.as_ptr() as *const c_char;
        let ptr = libc::dlopen(c_chars, libc::RTLD_LAZY);
        match ptr.is_null() {
            true => Err(CStr::from_ptr(libc::dlerror()).to_str().unwrap()),
            false => Ok(ptr),
        }
    }
}

/// Given the handle to the library and the symnbol, returns a pointer to
/// the function so that it can be called. Essentially a wrapper around
/// dlsym()
fn find_symbol(handle: *mut libc::c_void, func: &str) -> Result<*mut libc::c_void, &str> {
    unsafe {
        let c_str = CString::new(func).unwrap();
        let c_chars: *const c_char = c_str.as_ptr() as *const c_char;
        let fn_ptr = libc::dlsym(handle, c_chars);
        match fn_ptr.is_null() {
            true => Err(CStr::from_ptr(libc::dlerror()).to_str().unwrap()),
            false => Ok(fn_ptr),
        }
    }
}

fn call_func(fn_handle: *mut libc::c_void, args: &str) {
    unsafe {
        // hopefully I don't have to tell you this, but calling an unknown function in some random library
        // is incredibly dangerous. Be careful about what you call and with what parameters.
        // This is a good way to get your box exploited or to find undefined behavior.
        let func: unsafe extern "C" fn(...) -> usize = std::mem::transmute(fn_handle);

        let zoo = parse::str_to_cstring(args);
        let r = func(zoo);

        println!("{}", r);
    }
}

fn close_lib(lib_handle: *mut libc::c_void) -> i32 {
    unsafe { libc::dlclose(lib_handle) }
}
