use crate::prelude::*;

pub enum Data {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Flags32(u32),
    Flags64(u64),
    String(String),
    Url(String),
    Date(u32), // <-- TODO
    Time(u32), // <-- TODO
    GeoLocation(f32, f32),
    Pubkey(Pubkey),
    Array(Vec<Data>),
    Table(Vec<(Data, Data)>),
}

#[wasm_bindgen]
pub enum DataType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Url,
    Geo,
    Pubkey,
    Array,
    Table,
}

#[wasm_bindgen]
pub struct Schema(Vec<(String, DataType)>);

#[wasm_bindgen]
pub struct DataSet(Vec<Schema>);
