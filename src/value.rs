
use std::fmt::{Debug, Formatter, Result};
use std::collections::HashMap;

pub const TYPE_NULL: u16 = 0;
pub const TYPE_U8: u16 = 1;
pub const TYPE_I8: u16 = 2;
pub const TYPE_U16: u16 = 3;
pub const TYPE_I16: u16 = 4;
pub const TYPE_U32: u16 = 5;
pub const TYPE_I32: u16 = 6;
pub const TYPE_F32: u16 = 7;
pub const TYPE_F64: u16 = 8;
pub const TYPE_STR: u16 = 9;
pub const TYPE_RAW: u16 = 10;
pub const TYPE_MAP: u16 = 11;
pub const TYPE_VEC_U8: u16 = 21;
pub const TYPE_VEC_I8: u16 = 22;
pub const TYPE_VEC_U16: u16 = 23;
pub const TYPE_VEC_I16: u16 = 24;
pub const TYPE_VEC_U32: u16 = 25;
pub const TYPE_VEC_I32: u16 = 26;
pub const TYPE_VEC_F32: u16 = 27;
pub const TYPE_VEC_F64: u16 = 28;
pub const TYPE_VEC_STR: u16 = 29;
pub const TYPE_VEC_RAW: u16 = 30;
pub const TYPE_VEC_MAP: u16 = 31;

pub const STR_TYPE_NULL: &'static str = "null";
pub const STR_TYPE_U8: &'static str = "u8";
pub const STR_TYPE_I8: &'static str = "i8";
pub const STR_TYPE_U16: &'static str = "u16";
pub const STR_TYPE_I16: &'static str = "i16";
pub const STR_TYPE_U32: &'static str = "u32";
pub const STR_TYPE_I32: &'static str = "i32";
pub const STR_TYPE_F32: &'static str = "f32";
pub const STR_TYPE_F64: &'static str = "f64";
pub const STR_TYPE_STR: &'static str = "str";
pub const STR_TYPE_RAW: &'static str = "raw";
pub const STR_TYPE_MAP: &'static str = "map";
pub const STR_TYPE_VEC_U8: &'static str = "u8[]";
pub const STR_TYPE_VEC_I8: &'static str = "i8[]";
pub const STR_TYPE_VEC_U16: &'static str = "u16[]";
pub const STR_TYPE_VEC_I16: &'static str = "i16[]";
pub const STR_TYPE_VEC_U32: &'static str = "u32[]";
pub const STR_TYPE_VEC_I32: &'static str = "i32[]";
pub const STR_TYPE_VEC_F32: &'static str = "f32[]";
pub const STR_TYPE_VEC_F64: &'static str = "f64[]";
pub const STR_TYPE_VEC_STR: &'static str = "str[]";
pub const STR_TYPE_VEC_RAW: &'static str = "raw[]";
pub const STR_TYPE_VEC_MAP: &'static str = "map[]";

#[derive(PartialEq, Clone)]
pub enum Value {
    Null,
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    F32(f32),
    F64(f64),
    Str(String),
    Raw(Vec<u8>),
    Map(HashMap<String, Value>),
    VecU8(Vec<Value>),
    VecI8(Vec<Value>),
    VecU16(Vec<Value>),
    VecI16(Vec<Value>),
    VecU32(Vec<Value>),
    VecI32(Vec<Value>),
    VecF32(Vec<Value>),
    VecF64(Vec<Value>),
    VecStr(Vec<Value>),
    VecRaw(Vec<Value>),
    VecMap(Vec<Value>),
}

impl From<u8> for Value {
    fn from(value: u8) -> Value {
        Value::U8(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Value {
        Value::I8(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Value { 
        Value::U16(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Value {
        Value::I16(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Value {
        Value::U32(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Value {
        Value::I32(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Value {
        Value::F32(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value::F64(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::Str(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Value {
        Value::Raw(value)
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(value: HashMap<String, Value>) -> Value {
        Value::Map(value)
    }
}

impl Into<u8> for Value {
    fn into(self) -> u8 {
        match self {
            Value::U8(val) => val,
            _ => panic!("into error"),
        }
    }
}

impl Into<i8> for Value {
    fn into(self) -> i8 {
        match self {
            Value::I8(val) => val,
            _ => panic!("into error"),
        }
    }
}

impl Into<u16> for Value {
    fn into(self) -> u16 {
        match self {
            Value::U16(val) => val,
            _ => panic!("into error"),
        }
    }
}

impl Into<i16> for Value {
    fn into(self) -> i16 {
        match self {
            Value::I16(val) => val,
            _ => panic!("into error"),
        }
    }
}

impl Into<u32> for Value {
    fn into(self) -> u32 {
        match self {
            Value::U32(val) => val,
            _ => panic!("into error"),
        }
    }
}

impl Into<i32> for Value {
    fn into(self) -> i32 {
        match self {
            Value::I32(val) => val,
            _ => panic!("into error"),
        }
    }
}

impl Into<f32> for Value {
    fn into(self) -> f32 {
        match self {
            Value::F32(val) => val,
            _ => panic!("into error"),
        }
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        match self {
            Value::F64(val) => val,
            _ => panic!("into error"),
        }
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::Str(val) => val,
            _ => panic!("into error"),
        }
    }
}

impl Into<Vec<u8>> for Value {
    fn into(self) -> Vec<u8> {
        match self {
            Value::Raw(val) => val,
            _ => panic!("into error"),
        }
    }
}

impl Into<HashMap<String, Value>> for Value {
    fn into(self) -> HashMap<String, Value> {
        match self {
            Value::Map(val) => val,
            _ => panic!("into error"),
        }
    }
}

pub fn get_vec_elem_type(value: &Value) -> u16{
    match *value {
        Value::VecU8(_) => TYPE_U8,
        Value::VecI8(_) => TYPE_I8,
        Value::VecU16(_) => TYPE_U16,
        Value::VecI16(_) => TYPE_I16,
        Value::VecU32(_) => TYPE_U32,
        Value::VecI32(_) => TYPE_I32,
        Value::VecF32(_) => TYPE_F32,
        Value::VecF64(_) => TYPE_F64,
        Value::VecStr(_) => TYPE_STR,
        Value::VecRaw(_) => TYPE_RAW,
        Value::VecMap(_) => TYPE_MAP,
        _ => TYPE_NULL,
    }
}

pub fn get_value_type(value: &Value) -> u16 {
    match *value {
        Value::U8(_) => TYPE_U8,
        Value::I8(_) => TYPE_I8,
        Value::U16(_) => TYPE_U16,
        Value::I16(_) => TYPE_I16,
        Value::U32(_) => TYPE_U32,
        Value::I32(_) => TYPE_I32,
        Value::F32(_) => TYPE_F32,
        Value::F64(_) => TYPE_F64,
        Value::Str(_) => TYPE_STR,
        Value::Raw(_) => TYPE_RAW,
        Value::Map(_) => TYPE_MAP,
        Value::VecU8(_) => TYPE_VEC_U8,
        Value::VecI8(_) => TYPE_VEC_I8,
        Value::VecU16(_) => TYPE_VEC_U16,
        Value::VecI16(_) => TYPE_VEC_I16,
        Value::VecU32(_) => TYPE_VEC_U32,
        Value::VecI32(_) => TYPE_VEC_I32,
        Value::VecF32(_) => TYPE_VEC_F32,
        Value::VecF64(_) => TYPE_VEC_F64,
        Value::VecStr(_) => TYPE_VEC_STR,
        Value::VecRaw(_) => TYPE_VEC_RAW,
        Value::VecMap(_) => TYPE_VEC_MAP,
        _ => TYPE_NULL,
    }
}

pub fn get_type_by_name(name: &str) -> u16 {
    match name {
        STR_TYPE_NULL => TYPE_NULL,
        STR_TYPE_U8 => TYPE_U8,
        STR_TYPE_I8 => TYPE_I8,
        STR_TYPE_U16 => TYPE_U16,
        STR_TYPE_I16 => TYPE_I16,
        STR_TYPE_U32 => TYPE_U32,
        STR_TYPE_I32 => TYPE_I32,
        STR_TYPE_F32 => TYPE_F32,
        STR_TYPE_F64 => TYPE_F64,
        STR_TYPE_STR => TYPE_STR,
        STR_TYPE_RAW => TYPE_RAW,
        STR_TYPE_MAP => TYPE_MAP,
        STR_TYPE_VEC_U8 => TYPE_VEC_U8,
        STR_TYPE_VEC_I8 => TYPE_VEC_I8,
        STR_TYPE_VEC_U16 => TYPE_VEC_U16,
        STR_TYPE_VEC_I16 => TYPE_VEC_I16,
        STR_TYPE_VEC_U32 => TYPE_VEC_U32,
        STR_TYPE_VEC_I32 => TYPE_VEC_I32,
        STR_TYPE_VEC_F32 => TYPE_VEC_F32,
        STR_TYPE_VEC_F64 => TYPE_VEC_F64,
        STR_TYPE_VEC_STR => TYPE_VEC_STR,
        STR_TYPE_VEC_RAW => TYPE_VEC_RAW,
        STR_TYPE_VEC_MAP => TYPE_VEC_MAP,
        _ => TYPE_NULL,
    }
}

pub fn get_name_by_type(index: u16) -> &'static str {
    match index {
        TYPE_NULL => STR_TYPE_NULL,
        TYPE_U8 => STR_TYPE_U8,
        TYPE_I8 => STR_TYPE_I8,
        TYPE_U16 => STR_TYPE_U16,
        TYPE_I16 => STR_TYPE_I16,
        TYPE_U32 => STR_TYPE_U32,
        TYPE_I32 => STR_TYPE_I32,
        TYPE_F32 => STR_TYPE_F32,
        TYPE_F64 => STR_TYPE_F64,
        TYPE_STR => STR_TYPE_STR,
        TYPE_RAW => STR_TYPE_RAW,
        TYPE_MAP => STR_TYPE_MAP,
        TYPE_VEC_U8 => STR_TYPE_VEC_U8,
        TYPE_VEC_I8 => STR_TYPE_VEC_I8,
        TYPE_VEC_U16 => STR_TYPE_VEC_U16,
        TYPE_VEC_I16 => STR_TYPE_VEC_I16,
        TYPE_VEC_U32 => STR_TYPE_VEC_U32,
        TYPE_VEC_I32 => STR_TYPE_VEC_I32,
        TYPE_VEC_F32 => STR_TYPE_VEC_F32,
        TYPE_VEC_F64 => STR_TYPE_VEC_F64,
        TYPE_VEC_STR => STR_TYPE_VEC_STR,
        TYPE_VEC_RAW => STR_TYPE_VEC_RAW,
        TYPE_VEC_MAP => STR_TYPE_VEC_MAP,
        _ => STR_TYPE_NULL,
    }
}

impl Debug for Value {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match *self {
            Value::Null => write!(fmt, "null"),
            Value::U8(val) => write!(fmt, "u8({:?})", val),
            Value::I8(val) => write!(fmt, "i8({:?})", val),
            Value::U16(val) => write!(fmt, "u16({:?})", val),
            Value::I16(val) => write!(fmt, "i16({:?})", val),
            Value::U32(val) => write!(fmt, "u32({:?})", val),
            Value::I32(val) => write!(fmt, "i32({:?})", val),
            Value::F32(val) => write!(fmt, "f32({:?})", val),
            Value::F64(val) => write!(fmt, "f64({:?})", val),
            Value::Str(ref val) => write!(fmt, "str({:?})", val),
            Value::Raw(ref val) => write!(fmt, "raw({:?})", val),
            Value::Map(ref val) => write!(fmt, "str({:?})", val),
            Value::VecU8(ref val) => write!(fmt, "VecU8({:?})", val),
            Value::VecI8(ref val) => write!(fmt, "VecI8({:?})", val),
            Value::VecU16(ref val) => write!(fmt, "VecU16({:?})", val),
            Value::VecI16(ref val) => write!(fmt, "VecI16({:?})", val),
            Value::VecU32(ref val) => write!(fmt, "VecU32({:?})", val),
            Value::VecI32(ref val) => write!(fmt, "VecI32({:?})", val),
            Value::VecF32(ref val) => write!(fmt, "VecF32({:?})", val),
            Value::VecF64(ref val) => write!(fmt, "VecF64({:?})", val),
            Value::VecStr(ref val) => write!(fmt, "VecStr({:?})", val),
            Value::VecRaw(ref val) => write!(fmt, "VecRaw({:?})", val),
            Value::VecMap(ref val) => write!(fmt, "VecMap({:?})", val),
        }
    }
}