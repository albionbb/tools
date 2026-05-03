use photon_decoder::PhotonValue;
use std::collections::HashMap;

pub trait FromPhotonValue: Sized {
    fn from_photon(value: &PhotonValue) -> Option<Self>;
}

impl FromPhotonValue for bool {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl FromPhotonValue for u8 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Byte(b) => Some(*b),
            _ => None,
        }
    }
}

impl FromPhotonValue for i16 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Short(v) => Some(*v),
            _ => None,
        }
    }
}

impl FromPhotonValue for i32 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Int(v) => Some(*v),
            PhotonValue::Short(v) => Some(*v as i32),
            PhotonValue::Byte(v) => Some(*v as i32),
            _ => None,
        }
    }
}

impl FromPhotonValue for u32 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Long(v) => Some(*v as u32),
            PhotonValue::Int(v) => Some(*v as u32),
            PhotonValue::Short(v) => Some(*v as u32),
            PhotonValue::Byte(v) => Some(*v as u32),
            _ => None,
        }
    }
}

impl FromPhotonValue for i64 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Long(v) => Some(*v),
            PhotonValue::Int(v) => Some(*v as i64),
            PhotonValue::Short(v) => Some(*v as i64),
            PhotonValue::Byte(v) => Some(*v as i64),
            _ => None,
        }
    }
}

impl FromPhotonValue for u64 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Long(v) => Some(*v as u64),
            PhotonValue::Int(v) => Some(*v as u64),
            PhotonValue::Short(v) => Some(*v as u64),
            PhotonValue::Byte(v) => Some(*v as u64),
            _ => None,
        }
    }
}

impl FromPhotonValue for f32 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Float(v) => Some(*v),
            PhotonValue::Double(v) => Some(*v as f32),
            _ => None,
        }
    }
}

impl FromPhotonValue for f64 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Double(v) => Some(*v),
            PhotonValue::Float(v) => Some(*v as f64),
            _ => None,
        }
    }
}

impl FromPhotonValue for String {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

impl FromPhotonValue for Vec<String> {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Array(arr) | PhotonValue::ObjectArray(arr) => arr
                .iter()
                .map(|v| match v {
                    PhotonValue::String(s) => Some(s.clone()),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}

impl FromPhotonValue for Vec<u8> {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    PhotonValue::Byte(b) => Some(*b),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}

impl FromPhotonValue for Vec<u16> {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    PhotonValue::Short(s) => Some(*s as u16),
                    PhotonValue::Int(i) => Some(*i as u16),
                    PhotonValue::Byte(b) => Some(*b as u16),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}

impl FromPhotonValue for Vec<i16> {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    PhotonValue::Short(s) => Some(*s),
                    PhotonValue::Int(i) => Some(*i as i16),
                    PhotonValue::Byte(b) => Some(*b as i16),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}

pub fn get_param<T: FromPhotonValue>(params: &HashMap<u8, PhotonValue>, key: u8) -> Option<T> {
    params.get(&key).and_then(T::from_photon)
}
