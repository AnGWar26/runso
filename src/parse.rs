use std::{
    any::Any,
    ffi::{
        CString, c_char, c_float, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
        c_ushort, c_void,
    },
};

use anyhow::Result;

#[derive(Clone)]
pub enum CTypes {
    Int(c_int),
    Uint(c_uint),
    String(CString),
    Float(c_float),
    Char(c_char),
    Long(c_long),
    Schar(c_schar),
    Short(c_short),
    Uchar(c_uchar),
    Ulong(c_ulong),
    Ushort(c_ushort),
    Void(*mut c_void),
}

impl CTypes {
    pub fn unwrap_ctype(self) -> Box<dyn Any> {
        match self {
            CTypes::Int(x) => Box::new(x),
            CTypes::Uint(x) => Box::new(x),
            CTypes::String(x) => Box::new(x),
            CTypes::Float(x) => Box::new(x),
            CTypes::Char(x) => Box::new(x),
            CTypes::Long(x) => Box::new(x),
            CTypes::Schar(x) => Box::new(x),
            CTypes::Short(x) => Box::new(x),
            CTypes::Uchar(x) => Box::new(x),
            CTypes::Ulong(x) => Box::new(x),
            CTypes::Ushort(x) => Box::new(x),
            CTypes::Void(x) => Box::new(x),
        }
    }
}

pub fn parse_to_ctypes(input: Vec<&str>, types: Vec<&str>) -> Result<Vec<CTypes>> {
    let ctypes: Vec<CTypes> = input
        .iter()
        .zip(types)
        .map(|(i, t)| -> CTypes {
            match t {
                "int" => str_to_c_int(i).unwrap(),
                "uint" => str_to_c_uint(i).unwrap(),
                "string" => str_to_cstring(i).unwrap(),
                "float" => str_to_c_float(i).unwrap(),
                "char" => str_to_c_char(i).unwrap(),
                "long" => str_to_c_long(i).unwrap(),
                "schar" => str_to_c_schar(i).unwrap(),
                "short" => str_to_c_short(i).unwrap(),
                "uchar" => str_to_c_uchar(i).unwrap(),
                "ulong" => str_to_c_ulong(i).unwrap(),
                "ushort" => str_to_c_ushort(i).unwrap(),
                "void" => str_to_c_void().unwrap(),
                _ => panic!("parameter {:?} of type {:?} could not be parsed!", i, t),
            }
        })
        .collect();
    Ok(ctypes)
}

fn str_to_cstring(input: &str) -> Result<CTypes> {
    Ok(CTypes::String(CString::new(input)?))
}

fn str_to_c_int(input: &str) -> Result<CTypes> {
    Ok(CTypes::Int(input.parse::<i32>()?))
}

fn str_to_c_uint(input: &str) -> Result<CTypes> {
    Ok(CTypes::Uint(input.parse::<u32>()?))
}

fn str_to_c_float(input: &str) -> Result<CTypes> {
    Ok(CTypes::Float(input.parse::<f32>()?))
}

fn str_to_c_char(input: &str) -> Result<CTypes> {
    Ok(CTypes::Char(input.parse::<i8>()?))
}

fn str_to_c_schar(input: &str) -> Result<CTypes> {
    Ok(CTypes::Schar(input.parse::<i8>()?))
}

fn str_to_c_ushort(input: &str) -> Result<CTypes> {
    Ok(CTypes::Ushort(input.parse::<u16>()?))
}

fn str_to_c_ulong(input: &str) -> Result<CTypes> {
    Ok(CTypes::Ulong(input.parse::<u64>()?))
}

fn str_to_c_void() -> Result<CTypes> {
    Ok(CTypes::Void(std::ptr::null_mut()))
}

fn str_to_c_uchar(input: &str) -> Result<CTypes> {
    Ok(CTypes::Uchar(input.parse::<u8>()?))
}

fn str_to_c_short(input: &str) -> Result<CTypes> {
    Ok(CTypes::Short(input.parse::<i16>()?))
}

fn str_to_c_long(input: &str) -> Result<CTypes> {
    Ok(CTypes::Long(input.parse::<i64>()?))
}
