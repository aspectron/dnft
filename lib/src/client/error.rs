use thiserror::Error;
use wasm_bindgen::prelude::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    JsValue(JsValue),

    #[error("{0}")]
    String(String),

    #[error("Deserialization error: {0}")]
    SerdeWasmBindgen(#[from] serde_wasm_bindgen::Error),
}

impl From<JsError> for Error {
    fn from(err: JsError) -> Self {
        Error::JsValue(err.into())
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        match err {
            Error::JsValue(err) => err,
            _ => JsValue::from(err.to_string()),
        }
    }
}

// impl From<Error> for JsError {
//     fn from(err: Error) -> Self {
//         match err {
//             Error::JsError(err) => err,
//             _ => Error::JsError(err.to_string())
//         }
//     }
// }

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(value) //JsValue::from(&value.as_string().unwrap_or_else(||format!("{}", value))))
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::String(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::String(err)
    }
}
