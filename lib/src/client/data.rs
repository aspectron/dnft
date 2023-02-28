use super::error::Error;
use super::result::Result;
use crate::prelude::*;
// use crate::program;
// use crate::program::DataType;
use program::{DataType, Url, UrlType};
// use program::Url;

#[derive(Debug, Clone, TryFromJsValue)]
#[wasm_bindgen]
pub struct Data {
    // data_type: DataType,
    data: program::Data,
}

fn get_data_type(js_value: &JsValue) -> Result<DataType> {
    let dt = js_value
        .as_f64()
        .map(|v| v as u16)
        .ok_or_else(|| JsError::new("Unable to determine supplied schema data type."))?;
    let data_type: DataType = dt
        .try_into()
        .map_err(|_| JsError::new(&format!("Unknown data type value: {dt}")))?;
    Ok(data_type)
}

//     fn load_data(args: &JsValue) -> Result<program::Data> {
//         let mut vec = js_sys::Array::from(args).to_vec();
//         let data_type = get_data_type(&vec.remove(0))?;
//         let data: program::Data = (data_type, vec).try_into()?;
//         // Ok(Data {
//         Ok(data)
//         // })
// }

#[wasm_bindgen]
impl Data {
    #[wasm_bindgen(constructor, variadic)]
    pub fn try_new(args: &JsValue) -> Result<Data> {
        let mut vec = js_sys::Array::from(args).to_vec();
        let data_type = get_data_type(&vec.remove(0))?;
        let data: program::Data = (data_type, vec).try_into()?;
        Ok(Data { data })
    }

    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        // format!("{}", self.data)
        self.data.to_string()
    }
}

fn ensure_args(args: &[JsValue], n: usize) -> Result<()> {
    if args.len() != n {
        return Err("Expected {n} argument(s) for the supplied data type".into());
    }
    Ok(())
}

use num::NumCast;
fn get_number<T: NumCast + Copy + fmt::Display>(
    args: &[JsValue],
    min: T,
    max: T,
    // ) -> Result<T, JsError> {
) -> Result<T> {
    ensure_args(args, 1)?;
    let v = args
        .get(0)
        .unwrap()
        .as_f64()
        .ok_or_else(|| JsError::new("Supplied argument must be a number"))?;
    let min_: f64 = num::cast(min).unwrap();
    let max_: f64 = num::cast(max).unwrap();
    if v < min_ || v > max_ {
        return Err("number is out of range for the supplied type:  {min} => {v} <= {max}".into());
    }
    let v = num::cast(v).unwrap();
    Ok(v)
}

impl From<Data> for program::Data {
    fn from(value: Data) -> Self {
        value.data
    }
}

impl TryFrom<(DataType, Vec<JsValue>)> for program::Data {
    // type Error = kaizen::error::Error;
    // type Error = JsError;
    type Error = Error;
    // fn try_from((data_type, args): (DataType, Vec<JsValue>)) -> Result<Self, Self::Error> {
    // fn try_from((data_type, args): (DataType, Vec<JsValue>)) -> Result<Self, Self::Error> {
    fn try_from(
        (data_type, args): (DataType, Vec<JsValue>),
    ) -> std::result::Result<Self, Self::Error> {
        // use program::Data;
        let data = match data_type {
            DataType::Bool => {
                ensure_args(&args, 1)?;
                let v = args
                    .get(0)
                    .unwrap()
                    .as_bool()
                    .ok_or_else(|| JsError::new("Supplied argument must be a boolean"))?;
                program::Data::Bool(v)
            }
            DataType::u8 => program::Data::u8(get_number(&args, u8::MIN, u8::MAX)?),
            DataType::u16 => program::Data::u16(get_number(&args, u16::MIN, u16::MAX)?),
            DataType::u32 => program::Data::u32(get_number(&args, u32::MIN, u32::MAX)?),
            DataType::u64 => program::Data::u64(get_number(&args, u64::MIN, u64::MAX)?),
            DataType::i8 => program::Data::i8(get_number(&args, i8::MIN, i8::MAX)?),
            DataType::i16 => program::Data::i16(get_number(&args, i16::MIN, i16::MAX)?),
            DataType::i32 => program::Data::i32(get_number(&args, i32::MIN, i32::MAX)?),
            DataType::i64 => program::Data::i64(get_number(&args, i64::MIN, i64::MAX)?),
            DataType::f32 => program::Data::f32(get_number(&args, f32::MIN, f32::MAX)?),
            DataType::f64 => program::Data::f64(get_number(&args, f64::MIN, f64::MAX)?),
            DataType::String => {
                ensure_args(&args, 1)?;
                let v = args
                    .get(0)
                    .unwrap()
                    .as_string()
                    .ok_or_else(|| JsError::new("Supplied argument must be a string"))?;
                program::Data::String(v)
            }
            DataType::Url => {
                ensure_args(&args, 2)?;
                let url_str = args
                    .get(0)
                    .unwrap()
                    .as_string()
                    .ok_or_else(|| JsError::new("Supplied argument must be a string"))?;
                let kind = args.get(0).unwrap().clone();
                let kind: UrlType = from_value(kind).map_err(|err| {
                    JsError::new(&format!("Supplied argument must be a URL type: {err}"))
                })?;

                let url: Url = (kind, url_str.as_str()).into();
                program::Data::Url(url)
            }
            // DataType::ImageUrl => {
            //     ensure_args(&args, 1)?;
            //     let v = args
            //         .get(0)
            //         .unwrap()
            //         .as_string()
            //         .ok_or_else(|| JsError::new("Supplied argument must be a string"))?;
            //     program::Data::Url(Url::Image(v))
            // }
            // DataType::Geo => {
            //     ensure_args(&args, 1)?;
            //     let v: program::Geo = args
            //         .get(0)
            //         .unwrap()
            //         .try_into()
            //         .map_err(|err: String| JsError::new(&err))?;
            //     // let v = args.get(0).unwrap().as_string().ok_or_else(||JsError::new("Supplied argument must be a string"))?;
            //     program::Data::Geo(v)
            // }
            DataType::Pubkey => {
                ensure_args(&args, 1)?;
                program::Data::Pubkey(ref_from_abi!(Pubkey, args.get(0).unwrap())?)
            }
            // DataType::Array => {
            //     // todo!();
            //     ensure_args(&args, 1)?;
            //     let vec = js_sys::Array::from(args.get(0).unwrap()).to_vec();
            //     if vec.is_empty() {
            //         return Err("Supplied argument must be an array".into());
            //     }

            //     let mut list: Vec<program::Data> = Vec::new();
            //     for item in vec.iter() {
            //         let data: Data = item.try_into()?;
            //         list.push(data.data);
            //     }
            //     program::Data::Array(list)
            // }
            _ => {
                todo!()
            }
        };

        Ok(data)
    }
}

// impl From<String> for JsError {
//     fn from(err: String) -> Self {
//         JsError::new(&err)
//     }
// }

// fn get_instance<T>(js : &JsValue) -> Result<T,JsError>
// fn get_instance<T>(js : &JsValue) -> Result<T>
// where T: RefFromWasmAbi<Abi = u32> + Clone
// {

//     let ctor = ::js_sys::Reflect::get(js, &::wasm_bindgen::JsValue::from_str("constructor"))?;
//     log_info!("ctor: {:?}", ctor);
//     let name = ::js_sys::Reflect::get(&ctor, &::wasm_bindgen::JsValue::from_str("name"))?;
//     log_info!("name: {:?}", name);

//     let ptr = ::js_sys::Reflect::get(js, &::wasm_bindgen::JsValue::from_str("ptr"))?;
//         // .map_err(|err| JsError::new(&format!("{:?}", err)))?; //format!("{:?}", err))?;
//     let ptr_u32: u32 = ptr.as_f64().ok_or(::wasm_bindgen::JsValue::NULL)?
//         // .map_err(|err| JsError::new(&format!("{:?}", err)))? //format!("{:?}", err))?
//         as u32;
//     let instance_ref = unsafe { T::ref_from_abi(ptr_u32) };
//     Ok(instance_ref.clone())
// }
