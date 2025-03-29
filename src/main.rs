use core::str;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let lib = &args[1];
    let func = &args[2];
    let func_args = args[3].split(",").collect::<Vec<&str>>();
    let arg_types = args[4].split(",").collect::<Vec<&str>>();

    //"/usr/lib64/libc.so.6", "strlen", "VICTORY", char *s
    //TODO: error handle these
    let ptr = open_lib(lib).unwrap();
    let fn_handle = find_symbol(ptr, func).unwrap();

    call_func(fn_handle, func_args, arg_types);

    match close_lib(ptr) {
        0 => println!("Library closed successfully"),
        _ => println!("Something went wrong closing the library"),
    };
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

fn call_func(fn_handle: *mut libc::c_void, args: Vec<&str>, arg_types: Vec<&str>) {
    unsafe {
        // hopefully I don't have to tell you this, but calling an unknown function in some random library
        // is incredibly dangerous. Be careful about what you call and with what parameters.
        // This is a good way to get your box exploited or to find undefined behavior.
        let func: unsafe extern "C" fn(...) -> usize = std::mem::transmute(fn_handle);

        let parsed_args: Vec<parse::CTypes> = parse::parse_to_ctypes(args, arg_types).unwrap();

        let r = match parsed_args.len() {
            0 => func(),
            1 => func(&parsed_args[0].clone().unwrap_ctype()),
            2 => func(
                &parsed_args[0].clone().unwrap_ctype(),
                &parsed_args[1].clone().unwrap_ctype(),
            ),
            3 => func(
                &parsed_args[0].clone().unwrap_ctype(),
                &parsed_args[1].clone().unwrap_ctype(),
                &parsed_args[2].clone().unwrap_ctype(),
            ),
            _ => todo!(),
        };

        println!("{}", r);
    }
}

fn close_lib(lib_handle: *mut libc::c_void) -> i32 {
    unsafe { libc::dlclose(lib_handle) }
}
