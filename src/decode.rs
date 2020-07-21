use std::mem;
use std::io::Read;
use crate::error::ErrorKind;
use crate::value::*;
use crate::buffer::*;
use crate::error::RpResult;
use crate::config::{Config, Field};
use crate::macro_use::*;
use std::collections::HashMap;

pub fn decode_number(buffer: &mut Buffer, value_type: u16) -> RpResult<Value> {
    match value_type {
        TYPE_U8 => {
            let data: &mut [u8; 1] = &mut [0];
            try_read!(buffer.read(data), data.len());
            Ok(Value::from(data[0]))
        }
        TYPE_I8 => {
            let data: &mut [u8; 1] = &mut [0];
            try_read!(buffer.read(data), data.len());
            Ok(Value::from(data[0] as i8))
        }
        TYPE_U16 => {
            let data: &mut [u8; 2] = &mut [0, 0];
            try_read!(buffer.read(data), data.len());
            let value = unsafe { mem::transmute::<[u8; 2], u16>(*data)};
            Ok(Value::from(value))
        }
        TYPE_I16 => {
            let data: &mut [u8; 2] = &mut [0, 0];
            try_read!(buffer.read(data), data.len());
            let value = unsafe {mem::transmute::<[u8; 2], i16>(*data)};
            Ok(Value::from(value))
        }
        TYPE_U32 => {
            let data: &mut [u8; 4] = &mut [0, 0, 0, 0];
            try_read!(buffer.read(data), data.len());
            let value = unsafe {mem::transmute::<[u8; 4], u32>(*data)};
            Ok(Value::from(value))
        }
        TYPE_I32 => {
            let data: &mut [u8; 4] = &mut [0, 0, 0, 0];
            try_read!(buffer.read(data), data.len());
            let value = unsafe {mem::transmute::<[u8; 4], i32>(*data)};
            Ok(Value::from(value))
        }
        TYPE_F32 => {
            let data: &mut [u8; 4] = &mut [0, 0, 0, 0];
            try_read!(buffer.read(data), data.len());
            let value = unsafe {mem::transmute::<[u8; 4], f32>(*data)};
            Ok(Value::from(value))
        }
        TYPE_F64 => {
            let data: &mut [u8; 8] = &mut [0, 0, 0, 0, 0, 0, 0, 0];
            try_read!(buffer.read(data), data.len());
            let value = unsafe {mem::transmute::<[u8; 8], f64>(*data)};
            Ok(Value::from(value))
        }
        _ => {
            unreachable!("not other type");
        }
    }
}

pub fn decode_str_raw(buffer: &mut Buffer, value_type: u16) -> RpResult<Value> {
    match value_type {
        TYPE_STR => {
            let len : u16 = decode_number(buffer, TYPE_U16)?.into();
            if len == 0 {
                return Ok(Value::from(String::new()));
            }

            let mut data_vec = vec![0; len as usize];
            try_read!(buffer.read(&mut data_vec[..]), len as usize);
            let val = String::from_utf8(data_vec);
            if val.is_err() {
                fail!((ErrorKind::StringFormatError, "string format error"));
            }

            Ok(Value::from(val.ok().unwrap()))
        }
        TYPE_RAW => {
            let len: u16 = (decode_number(buffer, TYPE_U16))?.into();
            if len == 0 {
                return Ok(Value::from(vec![]));
            }
            let mut data_vec = vec![0; len as usize];
            try_read!(buffer.read(&mut data_vec[..]), len as usize);
            Ok(Value::from(data_vec))
        }
        _=> {
            unreachable!("not other type");
        }
    }
}

pub fn decode_map(buffer: &mut Buffer, config: &Config) -> RpResult<Value> {
    let mut map = HashMap::<String, Value>::new();
    loop {
        let field = read_field(buffer)?;
        if field.is_null_type() {
            return Ok(Value::from(map));
        }

        let sub_value = decode_field(buffer, config)?;
        let name = config.get_field_index_name(&field.index);
        if name.is_none() {
            continue;
        }

        let name = name.map(|s| s.clone()).unwrap();
        map.insert(name, sub_value);
    }
}

pub fn read_field(buffer: &mut Buffer) -> RpResult<Field> {
    let index = decode_number(buffer, TYPE_U16)?.into();

    let pattern = decode_number(buffer, TYPE_U16)?.into();
    Ok(Field{
        index,
        pattern: get_name_by_type(pattern).to_string(),
    })
}

pub fn decode_field(buffer: &mut Buffer, config: &Config) -> RpResult<Value> {
    let field = read_field(buffer)?;
    if field.is_null_type() {
        return Ok(Value::Null);
    }
    decode_by_field(buffer, config, &field)
}

fn decode_by_field(buffer: &mut Buffer, config: & Config, field: &Field) -> RpResult<Value> {
    let t = get_type_by_name(&*field.pattern);
    match t {
        TYPE_U8 | TYPE_I8 | TYPE_U16 | TYPE_I16 | TYPE_U32 | TYPE_I32 | TYPE_F32 | TYPE_F64 => {
            decode_number(buffer, t)
        }
        TYPE_STR | TYPE_RAW => decode_str_raw(buffer, t),
        TYPE_MAP => decode_map(buffer, config),
        TYPE_VEC_U8 => decode_array!(decode_field(buffer, config), Value::VecU8, Value::U8),
        TYPE_VEC_I8 => decode_array!(decode_field(buffer, config), Value::VecI8, Value::I8),
        TYPE_VEC_U16 => decode_array!(decode_field(buffer, config), Value::VecU16, Value::U16),
        TYPE_VEC_I16 => decode_array!(decode_field(buffer, config), Value::VecI16, Value::I16),
        TYPE_VEC_U32 => decode_array!(decode_field(buffer, config), Value::VecU32, Value::U32),
        TYPE_VEC_I32 => decode_array!(decode_field(buffer, config), Value::VecI32, Value::I32),
        TYPE_VEC_F32 => decode_array!(decode_field(buffer, config), Value::VecF32, Value::F32),
        TYPE_VEC_F64 => decode_array!(decode_field(buffer, config), Value::VecF64, Value::F64),
        TYPE_VEC_STR => decode_array!(decode_field(buffer, config), Value::VecStr, Value::Str),
        TYPE_VEC_RAW => decode_array!(decode_field(buffer, config), Value::VecRaw, Value::Raw),
        TYPE_VEC_MAP => decode_array!(decode_field(buffer, config), Value::VecMap, Value::Map),
        TYPE_NULL => Ok(Value::Null),
        _ => fail!((ErrorKind::TypeNotMatchError, "must match type")),
    }
}

pub fn decode_proto(buffer: &mut Buffer, config: &Config) -> RpResult<(String, Vec<Value>)> {
    let name = decode_str_raw(buffer, TYPE_STR).unwrap().into();

    let mut value: Vec<Value> = vec![];
    loop {
        let sub_value = decode_field(buffer, config)?;
        match sub_value {
            Value::Null => break,
            _ => (),
        }
        value.push(sub_value);
    }

    let proto = config.get_proto_by_name(&name);
    match proto {
        Some(val) => {
            if val.args.len() != value.len() {
                fail!((ErrorKind::TypeNotMatchError, "must match type"));
            }
        }
        _ => {
            fail!((ErrorKind::TypeNotMatchError, "must match type"));
        }
    }
    Ok((name, value))
}