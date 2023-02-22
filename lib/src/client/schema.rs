use super::result::Result;
use crate::prelude::*;
// use super::error::Error;
use crate::program::*;

pub type Description = String;
pub type Name = String;

#[derive(Debug, Clone, TryFromJsValue, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct Field {
    #[serde(rename = "type")]
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

    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(js_name = "dataType")]
    pub fn data_type(&self) -> DataType {
        self.data_type
    }

    #[wasm_bindgen]
    pub fn description(&self) -> String {
        self.description.clone()
    }

    fn try_from_any(v: &JsValue) -> Result<Self> {
        if let Ok(f) = Field::try_from(v) {
            Ok(f)
        } else {
            Ok(from_value(v.clone())?)
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*
        f.debug_struct("Field")
            .field("type", &format!("{:?}", self.data_type))
            .field("name", &format!("{}", self.name))
            .field("description", &format!("{}", self.description));
        Ok(())
        */

        write!(
            f,
            "Field {{ type: {:?},  name:'{}', descr:'{}' }}",
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
            // .map(|f| f.try_into())
            .map(|f| Field::try_from_any(f))
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

    pub fn fields(&self) -> js_sys::Array {
        let result = js_sys::Array::new(); //self.fields.len() as u32);
        for field in self.fields.clone() {
            result.push(&field.into());
        }
        result
    }
}

impl Schema {
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }
}

impl From<Schema> for MintCreationArgs {
    fn from(value: Schema) -> Self {
        let mut data_types = program::DataTypes::default();
        let mut names = vec![];
        let mut descriptions = vec![];

        for field in value.fields {
            data_types.push(field.data_type());
            names.push(field.name());
            descriptions.push(field.description());
        }

        Self {
            data_types: Some(data_types),
            names: Some(names),
            descriptions: Some(descriptions),
        }
    }
}
