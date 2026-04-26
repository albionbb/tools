use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::{self, Read};

/// Protocol18 type codes
#[allow(dead_code)]
pub(crate) const TYPE_UNKNOWN: u8 = 0;
pub(crate) const TYPE_BOOLEAN: u8 = 2;
pub(crate) const TYPE_BYTE: u8 = 3;
pub(crate) const TYPE_SHORT: u8 = 4;
pub(crate) const TYPE_FLOAT: u8 = 5;
pub(crate) const TYPE_DOUBLE: u8 = 6;
pub(crate) const TYPE_STRING: u8 = 7;
pub(crate) const TYPE_NULL: u8 = 8;
pub(crate) const TYPE_COMPRESSED_INT: u8 = 9;
pub(crate) const TYPE_COMPRESSED_LONG: u8 = 10;
pub(crate) const TYPE_INT1: u8 = 11;
pub(crate) const TYPE_INT1_NEG: u8 = 12;
pub(crate) const TYPE_INT2: u8 = 13;
pub(crate) const TYPE_INT2_NEG: u8 = 14;
pub(crate) const TYPE_LONG1: u8 = 15;
pub(crate) const TYPE_LONG1_NEG: u8 = 16;
pub(crate) const TYPE_LONG2: u8 = 17;
pub(crate) const TYPE_LONG2_NEG: u8 = 18;
pub(crate) const TYPE_CUSTOM: u8 = 19;
pub(crate) const TYPE_DICTIONARY: u8 = 20;
pub(crate) const TYPE_HASHTABLE: u8 = 21;
pub(crate) const TYPE_OBJECT_ARRAY: u8 = 23;
pub(crate) const TYPE_OPERATION_REQUEST: u8 = 24;
pub(crate) const TYPE_OPERATION_RESPONSE: u8 = 25;
pub(crate) const TYPE_EVENT_DATA: u8 = 26;
pub(crate) const TYPE_BOOL_FALSE: u8 = 27;
pub(crate) const TYPE_BOOL_TRUE: u8 = 28;
pub(crate) const TYPE_SHORT_ZERO: u8 = 29;
pub(crate) const TYPE_INT_ZERO: u8 = 30;
pub(crate) const TYPE_LONG_ZERO: u8 = 31;
pub(crate) const TYPE_FLOAT_ZERO: u8 = 32;
pub(crate) const TYPE_DOUBLE_ZERO: u8 = 33;
pub(crate) const TYPE_BYTE_ZERO: u8 = 34;
pub(crate) const TYPE_ARRAY: u8 = 0x40;
pub(crate) const CUSTOM_TYPE_SLIM_BASE: u8 = 0x80;

/// A decoded Protocol18 value.
#[derive(Debug, Clone, PartialEq)]
pub enum PhotonValue {
    Null,
    Bool(bool),
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Array(Vec<PhotonValue>),
    Dictionary(HashMap<PhotonValue, PhotonValue>),
    ObjectArray(Vec<PhotonValue>),
    Custom {
        id: u8,
        data: Vec<u8>,
    },
    OperationRequest {
        op_code: u8,
        params: HashMap<u8, PhotonValue>,
    },
    OperationResponse {
        op_code: u8,
        return_code: i16,
        debug_message: String,
        params: HashMap<u8, PhotonValue>,
    },
    EventData {
        code: u8,
        params: HashMap<u8, PhotonValue>,
    },
}

impl std::hash::Hash for PhotonValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        use PhotonValue::*;
        match self {
            Null => 0.hash(state),
            Bool(v) => {
                1.hash(state);
                v.hash(state);
            }
            Byte(v) => {
                2.hash(state);
                v.hash(state);
            }
            Short(v) => {
                3.hash(state);
                v.hash(state);
            }
            Int(v) => {
                4.hash(state);
                v.hash(state);
            }
            Long(v) => {
                5.hash(state);
                v.hash(state);
            }
            Float(v) => {
                6.hash(state);
                v.to_bits().hash(state);
            }
            Double(v) => {
                7.hash(state);
                v.to_bits().hash(state);
            }
            String(v) => {
                8.hash(state);
                v.hash(state);
            }
            Array(v) => {
                9.hash(state);
                v.hash(state);
            }
            Dictionary(v) => {
                10.hash(state);
                v.len().hash(state);
            }
            ObjectArray(v) => {
                11.hash(state);
                v.hash(state);
            }
            Custom { id, data } => {
                12.hash(state);
                id.hash(state);
                data.hash(state);
            }
            OperationRequest { op_code, params } => {
                13.hash(state);
                op_code.hash(state);
                params.len().hash(state);
            }
            OperationResponse {
                op_code,
                return_code,
                debug_message,
                params,
            } => {
                14.hash(state);
                op_code.hash(state);
                return_code.hash(state);
                debug_message.hash(state);
                params.len().hash(state);
            }
            EventData { code, params } => {
                15.hash(state);
                code.hash(state);
                params.len().hash(state);
            }
        }
    }
}

impl Eq for PhotonValue {}

/// Deserialize a parameter table from raw bytes.
pub fn deserialize_parameter_table(data: &[u8]) -> HashMap<u8, PhotonValue> {
    let mut cursor = io::Cursor::new(data);
    read_parameter_table(&mut cursor)
}

pub fn read_parameter_table(cursor: &mut io::Cursor<&[u8]>) -> HashMap<u8, PhotonValue> {
    let count = read_count(cursor) as usize;
    let mut params = HashMap::with_capacity(count);
    for _ in 0..count {
        let Ok(key) = cursor.read_u8() else { break };
        let Ok(tc) = cursor.read_u8() else { break };
        params.insert(key, deserialize(cursor, tc));
    }
    params
}

/// Deserialize a single Protocol18 value given its type code.
pub fn deserialize(cursor: &mut io::Cursor<&[u8]>, tc: u8) -> PhotonValue {
    if tc >= CUSTOM_TYPE_SLIM_BASE {
        return deserialize_custom(cursor, tc);
    }
    match tc {
        TYPE_UNKNOWN | TYPE_NULL => PhotonValue::Null,
        TYPE_BOOLEAN => PhotonValue::Bool(cursor.read_u8().unwrap_or(0) != 0),
        TYPE_BYTE => PhotonValue::Byte(cursor.read_u8().unwrap_or(0)),
        TYPE_SHORT => PhotonValue::Short(cursor.read_i16::<LittleEndian>().unwrap_or(0)),
        TYPE_FLOAT => PhotonValue::Float(cursor.read_f32::<LittleEndian>().unwrap_or(0.0)),
        TYPE_DOUBLE => PhotonValue::Double(cursor.read_f64::<LittleEndian>().unwrap_or(0.0)),
        TYPE_STRING => PhotonValue::String(read_string(cursor)),
        TYPE_COMPRESSED_INT => PhotonValue::Int(read_compressed_int32(cursor)),
        TYPE_COMPRESSED_LONG => PhotonValue::Long(read_compressed_int64(cursor)),
        TYPE_INT1 => PhotonValue::Int(cursor.read_u8().unwrap_or(0) as i32),
        TYPE_INT1_NEG => PhotonValue::Int(-(cursor.read_u8().unwrap_or(0) as i32)),
        TYPE_INT2 => PhotonValue::Int(cursor.read_u16::<LittleEndian>().unwrap_or(0) as i32),
        TYPE_INT2_NEG => PhotonValue::Int(-(cursor.read_u16::<LittleEndian>().unwrap_or(0) as i32)),
        TYPE_LONG1 => PhotonValue::Long(cursor.read_u8().unwrap_or(0) as i64),
        TYPE_LONG1_NEG => PhotonValue::Long(-(cursor.read_u8().unwrap_or(0) as i64)),
        TYPE_LONG2 => PhotonValue::Long(cursor.read_u16::<LittleEndian>().unwrap_or(0) as i64),
        TYPE_LONG2_NEG => {
            PhotonValue::Long(-(cursor.read_u16::<LittleEndian>().unwrap_or(0) as i64))
        }
        TYPE_CUSTOM => deserialize_custom(cursor, 0),
        TYPE_DICTIONARY => PhotonValue::Dictionary(deserialize_dictionary(cursor)),
        TYPE_HASHTABLE => PhotonValue::Dictionary(deserialize_dictionary(cursor)),
        TYPE_OBJECT_ARRAY => PhotonValue::ObjectArray(deserialize_object_array(cursor)),
        TYPE_OPERATION_REQUEST => deserialize_operation_request_inner(cursor),
        TYPE_OPERATION_RESPONSE => deserialize_operation_response_inner(cursor),
        TYPE_EVENT_DATA => deserialize_event_data_inner(cursor),
        TYPE_BOOL_FALSE => PhotonValue::Bool(false),
        TYPE_BOOL_TRUE => PhotonValue::Bool(true),
        TYPE_SHORT_ZERO => PhotonValue::Short(0),
        TYPE_INT_ZERO => PhotonValue::Int(0),
        TYPE_LONG_ZERO => PhotonValue::Long(0),
        TYPE_FLOAT_ZERO => PhotonValue::Float(0.0),
        TYPE_DOUBLE_ZERO => PhotonValue::Double(0.0),
        TYPE_BYTE_ZERO => PhotonValue::Byte(0),
        TYPE_ARRAY => PhotonValue::Array(deserialize_nested_array(cursor)),
        _ => {
            if tc & TYPE_ARRAY == TYPE_ARRAY {
                PhotonValue::Array(deserialize_typed_array(cursor, tc & !TYPE_ARRAY))
            } else {
                PhotonValue::Null
            }
        }
    }
}

fn deserialize_typed_array(cursor: &mut io::Cursor<&[u8]>, elem_type: u8) -> Vec<PhotonValue> {
    let size = read_count(cursor) as usize;
    match elem_type {
        TYPE_BOOLEAN => {
            let packed_bytes = size.div_ceil(8);
            let mut packed = vec![0u8; packed_bytes];
            let _ = cursor.read_exact(&mut packed);
            (0..size)
                .map(|i| PhotonValue::Bool((packed[i / 8] & (1 << (i % 8))) != 0))
                .collect()
        }
        TYPE_BYTE => {
            let mut data = vec![0u8; size];
            let _ = cursor.read_exact(&mut data);
            data.into_iter().map(PhotonValue::Byte).collect()
        }
        TYPE_SHORT => (0..size)
            .map(|_| PhotonValue::Short(cursor.read_i16::<LittleEndian>().unwrap_or(0)))
            .collect(),
        TYPE_FLOAT => (0..size)
            .map(|_| PhotonValue::Float(cursor.read_f32::<LittleEndian>().unwrap_or(0.0)))
            .collect(),
        TYPE_DOUBLE => (0..size)
            .map(|_| PhotonValue::Double(cursor.read_f64::<LittleEndian>().unwrap_or(0.0)))
            .collect(),
        TYPE_STRING => (0..size)
            .map(|_| PhotonValue::String(read_string(cursor)))
            .collect(),
        TYPE_CUSTOM => {
            let custom_type_id = cursor.read_u8().unwrap_or(0);
            (0..size)
                .map(|_| deserialize_custom_payload(cursor, custom_type_id, false))
                .collect()
        }
        TYPE_DICTIONARY => (0..size)
            .map(|_| PhotonValue::Dictionary(deserialize_dictionary(cursor)))
            .collect(),
        TYPE_HASHTABLE => (0..size)
            .map(|_| PhotonValue::Dictionary(deserialize_dictionary(cursor)))
            .collect(),
        TYPE_COMPRESSED_INT => (0..size)
            .map(|_| PhotonValue::Int(read_compressed_int32(cursor)))
            .collect(),
        TYPE_COMPRESSED_LONG => (0..size)
            .map(|_| PhotonValue::Long(read_compressed_int64(cursor)))
            .collect(),
        _ => (0..size).map(|_| deserialize(cursor, elem_type)).collect(),
    }
}

fn deserialize_nested_array(cursor: &mut io::Cursor<&[u8]>) -> Vec<PhotonValue> {
    let size = read_count(cursor) as usize;
    let tc = cursor.read_u8().unwrap_or(0);
    (0..size).map(|_| deserialize(cursor, tc)).collect()
}

fn deserialize_object_array(cursor: &mut io::Cursor<&[u8]>) -> Vec<PhotonValue> {
    let size = read_count(cursor) as usize;
    let mut result = Vec::with_capacity(size);
    for _ in 0..size {
        let tc = cursor.read_u8().unwrap_or(0);
        result.push(deserialize(cursor, tc));
    }
    result
}

fn deserialize_dictionary(cursor: &mut io::Cursor<&[u8]>) -> HashMap<PhotonValue, PhotonValue> {
    let key_tc = cursor.read_u8().unwrap_or(0);
    let val_tc = cursor.read_u8().unwrap_or(0);
    let count = read_count(cursor) as usize;
    let mut out = HashMap::with_capacity(count);
    for _ in 0..count {
        let kt = if key_tc == 0 {
            cursor.read_u8().unwrap_or(0)
        } else {
            key_tc
        };
        let vt = if val_tc == 0 {
            cursor.read_u8().unwrap_or(0)
        } else {
            val_tc
        };
        let key = deserialize(cursor, kt);
        let val = deserialize(cursor, vt);
        out.insert(key, val);
    }
    out
}

fn deserialize_custom(cursor: &mut io::Cursor<&[u8]>, gp_type: u8) -> PhotonValue {
    let is_slim = gp_type >= CUSTOM_TYPE_SLIM_BASE;
    let custom_id = if is_slim {
        gp_type & 0x7F
    } else {
        cursor.read_u8().unwrap_or(0)
    };
    deserialize_custom_payload(cursor, custom_id, is_slim)
}

fn deserialize_custom_payload(
    cursor: &mut io::Cursor<&[u8]>,
    custom_id: u8,
    is_slim: bool,
) -> PhotonValue {
    let size = read_count(cursor) as usize;
    let remaining = cursor.get_ref().len() - cursor.position() as usize;
    if size > remaining {
        if is_slim {
            let mut data = vec![0u8; remaining];
            let _ = cursor.read_exact(&mut data);
            return PhotonValue::Custom {
                id: custom_id,
                data,
            };
        }
        return PhotonValue::Null;
    }
    let mut data = vec![0u8; size];
    let _ = cursor.read_exact(&mut data);
    PhotonValue::Custom {
        id: custom_id,
        data,
    }
}

fn deserialize_operation_request_inner(cursor: &mut io::Cursor<&[u8]>) -> PhotonValue {
    let op_code = cursor.read_u8().unwrap_or(0);
    let params = read_parameter_table(cursor);
    PhotonValue::OperationRequest { op_code, params }
}

fn deserialize_operation_response_inner(cursor: &mut io::Cursor<&[u8]>) -> PhotonValue {
    let op_code = cursor.read_u8().unwrap_or(0);
    let return_code = cursor.read_i16::<LittleEndian>().unwrap_or(0);
    let mut debug_message = String::new();
    if cursor.position() < cursor.get_ref().len() as u64 {
        let tc = cursor.read_u8().unwrap_or(0);
        if let PhotonValue::String(s) = deserialize(cursor, tc) {
            debug_message = s;
        }
    }
    let params = read_parameter_table(cursor);
    PhotonValue::OperationResponse {
        op_code,
        return_code,
        debug_message,
        params,
    }
}

fn deserialize_event_data_inner(cursor: &mut io::Cursor<&[u8]>) -> PhotonValue {
    let code = cursor.read_u8().unwrap_or(0);
    let params = read_parameter_table(cursor);
    PhotonValue::EventData { code, params }
}

// ── low-level readers ────────────────────────────────────────────────────────

fn read_string(cursor: &mut io::Cursor<&[u8]>) -> String {
    let length = read_compressed_uint32(cursor) as usize;
    if length == 0 || length > cursor.get_ref().len() - cursor.position() as usize {
        return String::new();
    }
    let mut buf = vec![0u8; length];
    let _ = cursor.read_exact(&mut buf);
    String::from_utf8_lossy(&buf).into_owned()
}

fn read_count(cursor: &mut io::Cursor<&[u8]>) -> u32 {
    read_compressed_uint32(cursor)
}

pub fn read_compressed_uint32(cursor: &mut io::Cursor<&[u8]>) -> u32 {
    let mut value: u32 = 0;
    let mut shift: u32 = 0;
    loop {
        let b = match cursor.read_u8() {
            Ok(v) => v,
            Err(_) => return 0,
        };
        value |= (b as u32 & 0x7F) << shift;
        if b & 0x80 == 0 {
            return value;
        }
        shift += 7;
        if shift >= 35 {
            return 0;
        }
    }
}

#[allow(dead_code)]
pub fn read_compressed_uint64(cursor: &mut io::Cursor<&[u8]>) -> u64 {
    let mut value: u64 = 0;
    let mut shift: u32 = 0;
    loop {
        let b = match cursor.read_u8() {
            Ok(v) => v,
            Err(_) => return 0,
        };
        value |= (b as u64 & 0x7F) << shift;
        if b & 0x80 == 0 {
            return value;
        }
        shift += 7;
        if shift >= 70 {
            return 0;
        }
    }
}

pub fn read_compressed_int32(cursor: &mut io::Cursor<&[u8]>) -> i32 {
    let v = read_compressed_uint32(cursor);
    ((v >> 1) as i32) ^ (-((v & 1) as i32))
}

pub fn read_compressed_int64(cursor: &mut io::Cursor<&[u8]>) -> i64 {
    let v = read_compressed_uint64(cursor);
    ((v >> 1) as i64) ^ (-((v & 1) as i64))
}
