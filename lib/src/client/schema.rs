use super::result::Result;
use crate::prelude::*;
// use super::error::Error;
use crate::program::*;

pub type Description = String;
pub type Name = String;

#[derive(Debug, Clone, TryFromJsValue)]
#[wasm_bindgen]
pub struct Field {
    data_type: DataType,
    name: String,
    description: String,
}

#[wasm_bindgen]
impl Field {
    #[wasm_bindgen(constructor)]
    pub fn new(data_type: DataType, name: String, description: String) -> Self {
        Self {
            data_type,
            name,
            description,
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "type: {:?} name:'{}' descr:'{}'",
            self.data_type, self.name, self.description
        )
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Schema {
    fields: Vec<Field>,
}

#[wasm_bindgen]
impl Schema {
    #[wasm_bindgen(constructor)]
    // pub fn new(fields : Vec<Field>) -> Self {
    // pub fn try_new(array: js_sys::Array) -> Result<Schema, JsError> {
    pub fn try_new(array: js_sys::Array) -> Result<Schema> {
        let fields = array
            .to_vec()
            .iter()
            .map(|f| f.try_into())
            // .collect::<Result<Vec<Field>, _>>()
            .collect::<std::result::Result<Vec<Field>, _>>()
            .map_err(|_| JsError::new("Unable to convert array item to `Field` structure."))?;

        Ok(Schema { fields })
    }

    pub fn display(&self) {
        for field in self.fields.iter() {
            log_info!("{}", field);
        }
    }
}
