use std::io::Write;
use std::mem;
use crate::value::*;
use crate::{Buffer};
use crate::error::*;
use crate::config::{Config, Field};

fn write_str_field(buffer: &mut Buffer, pattern: &str) ->RpResult<bool> {
    encode_number(buffer, &Value::from(0 as u16))?;
    encode_number(buffer, &Value::U16(get_type_by_name(pattern)))?;
    Ok(true)
}

fn append_and_align(buffer: &mut Buffer, val: &[u8]) -> RpResult<()> {
    let _add = match val.len() % 2 {
        0 => 0,
        val => 2 - val,
    };
    buffer.write(val)?;
    Ok(())
}

pub fn encode_number(buffer: &mut Buffer, value: &Value) -> RpResult<()> {
    match *value {
        Value::U8(val) => {
            buffer.write(unsafe { &mem::transmute::<u8, [u8; 1]>(val) })?;
        }
        Value::I8(val) => {
            buffer.write(unsafe { &mem::transmute::<i8, [u8; 1]>(val) })?;
        }
        Value::U16(val) => {
            buffer.write(unsafe { &mem::transmute::<u16, [u8; 2]>(val.to_le()) })?;
        }
        Value::I16(val) => {
            buffer.write(unsafe { &mem::transmute::<i16, [u8; 2]>(val.to_le()) })?;
        }
        Value::U32(val) => {
            buffer.write(unsafe { &mem::transmute::<u32, [u8; 4]>(val.to_le()) })?;
        }
        Value::I32(val) => {
            buffer.write(unsafe { &mem::transmute::<i32, [u8; 4]>(val.to_le()) })?;
        }
        Value::F32(val) => {
            let val = (val * 1000.0) as i32;
            buffer.write(unsafe { &mem::transmute::<i32, [u8; 4]>(val.to_le()) })?;
        }
        Value::F64(val) => {
            let val = (val * 1000.0) as i64;
            buffer.write(unsafe {&mem::transmute::<i64, [u8; 8]>(val.to_le())})?;
        }
        _ => unreachable!("encode_number only"),
    }
    Ok(())
}

pub fn encode_str_raw(buffer: &mut Buffer, value: &Value) -> RpResult<()> {
    match *value {
        Value::Str(ref val) => {
            encode_number(buffer, &Value::U16(val.len() as u16))?;
            append_and_align(buffer, &val.as_bytes()[..])?;
        }
        Value::Raw(ref val) => {
            encode_number(buffer, &Value::U16(val.len() as u16))?;
            append_and_align(buffer, &val[..])?;
        }
        _ => unreachable!("encode_str_raw only"),
    }
    Ok(())
}

pub fn encode_map(buffer: &mut Buffer, config: &Config, value: &Value) -> RpResult<()> {
    match *value {
        Value::Map(ref val) => {
            for (name, sub_value) in val {
                if write_field(buffer, config.get_field_by_name(name))? {
                    encode_field(buffer, config, sub_value)?;
                }
            }
            write_str_field(buffer, STR_TYPE_NULL)?;
        }
        _ => unreachable!("encode_map only"),
    }
    Ok(())
}


pub fn write_field(buffer: &mut Buffer, field: Option<&Field>) -> RpResult<bool> {
    if field.is_none() {
        return Ok(false);
    }
    let field = field.unwrap();
    encode_number(buffer, &Value::U16(field.index))?;
    encode_number(buffer, &Value::U16(get_type_by_name(&field.pattern)))?;
    Ok(true)
}

pub fn encode_field(buffer: &mut Buffer, config: &Config, value: &Value) -> RpResult<()> {
    write_str_field(buffer, get_name_by_type(get_value_type(value)))?;
    match *value {
        Value::U8(_) |
        Value::I8(_) |
        Value::U16(_) |
        Value::I16(_) |
        Value::U32(_) |
        Value::I32(_) |
        Value::F32(_) |
        Value::F64(_) => {
            encode_number(buffer, value)?;
        }
        Value::Str(_) | Value::Raw(_) => {
            encode_str_raw(buffer, value)?;
        }
        Value::Map(_) => {
            encode_map(buffer, config, value)?;
        }
        Value::Null => {}
        Value::VecU8(ref val) |
        Value::VecI8(ref val) |
        Value::VecU16(ref val) |
        Value::VecI16(ref val) |
        Value::VecU32(ref val) |
        Value::VecI32(ref val) |
        Value::VecF32(ref val) |
        Value::VecF64(ref val) |
        Value::VecStr(ref val) |
        Value::VecRaw(ref val) |
        Value::VecMap(ref val) => {
            let must_type = get_vec_elem_type(value);
            for v in val {
                check_vailed!(v, must_type);
                encode_field(buffer, config, v)?;
            }
            write_str_field(buffer, STR_TYPE_NULL)?;
        }
    }
    Ok(())
}

pub fn encode_proto(buffer: &mut Buffer,
                    config: &Config,
                    name: &String,
                    infos: Vec<Value>)
                    -> RpResult<()> {
    let proto = config.get_proto_by_name(name);
    ensure!(proto.is_some(),
            (ErrorKind::MissingError, "missing the name protocol"));
    let proto = proto.unwrap();
    ensure!(proto.args.len() == infos.len(),
            (ErrorKind::TypeNotMatchError, "the data num not match protocol args num"));
    encode_str_raw(buffer, &Value::Str(name.clone()))?;
    for info in &infos {
        encode_field(buffer, config, info)?;
    }
    write_str_field(buffer, STR_TYPE_NULL)?;
    Ok(())
}