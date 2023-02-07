use crate::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Data {
    Bool(bool),
    u8(u8),
    u16(u16),
    u32(u32),
    u64(u64),
    i8(i8),
    i16(i16),
    i32(i32),
    i64(i64),
    f32(f32),
    f64(f64),
    Flags32(u32),
    Flags64(u64),
    String(String),
    Url(String),
    Date(u32), // <-- TODO
    Time(u32), // <-- TODO
    Geo(Geo),
    Pubkey(Pubkey),
    Array(Vec<Data>),
    Table(Vec<(Data, Data)>),
}

u16_try_from! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy)]
    #[wasm_bindgen]
    #[repr(u16)]
    pub enum DataType {
        Bool,
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,
        String,
        Url,
        Geo,
        Pubkey,
        Array,
        Table,
    }
}

#[derive(Debug, Clone, TryFromJsValue)]
#[wasm_bindgen]
pub struct Geo {
    pub latitude: f64,
    pub longitude: f64,
}

#[wasm_bindgen]
impl Geo {
    #[wasm_bindgen(constructor)]
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[cfg(not(target_os = "solana"))]
impl fmt::Display for super::Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // use super::Data;
        match self {
            Data::Bool(v) => {
                write!(f, "{v}")
            }
            Data::u8(v) => {
                write!(f, "{v}")
            }
            Data::u16(v) => {
                write!(f, "{v}")
            }
            Data::u32(v) => {
                write!(f, "{v}")
            }
            Data::u64(v) => {
                write!(f, "{v}")
            }
            Data::i8(v) => {
                write!(f, "{v}")
            }
            Data::i16(v) => {
                write!(f, "{v}")
            }
            Data::i32(v) => {
                write!(f, "{v}")
            }
            Data::i64(v) => {
                write!(f, "{v}")
            }
            Data::f32(v) => {
                write!(f, "{v}")
            }
            Data::f64(v) => {
                write!(f, "{v}")
            }
            Data::String(v) => {
                write!(f, "{v}")
            }
            Data::Url(v) => {
                write!(f, "{v}")
            }
            Data::Geo(v) => {
                write!(f, "{},{}", v.latitude, v.longitude)
            }
            Data::Array(v) => {
                for item in v {
                    writeln!(f, "\t{item}").unwrap();
                }
                Ok(())
            }
            _ => write!(f, "{self:?}"),
        }
    }
}
