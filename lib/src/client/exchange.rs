use crate::prelude::*;
use program::exchange::SaleType as Inner;

#[derive(Debug, Clone, TryFromJsValue)]
#[wasm_bindgen]
pub struct SaleType {
    inner: Inner,
}
#[wasm_bindgen]
impl SaleType {
    pub fn auction() -> Self {
        Self {
            inner: Inner::Auction,
        }
    }
    pub fn barter() -> Self {
        Self {
            inner: Inner::Barter,
        }
    }
    pub fn none() -> Self {
        Self { inner: Inner::None }
    }
    pub fn raffle() -> Self {
        Self {
            inner: Inner::Raffle,
        }
    }
    pub fn rent() -> Self {
        Self { inner: Inner::Rent }
    }

    #[wasm_bindgen(js_name = "fromStr")]
    pub fn from_string(str: &str) -> JsValue {
        match Self::try_from(str) {
            Ok(v) => v.into(),
            Err(_) => JsValue::UNDEFINED,
        }
    }
}

impl TryFrom<&str> for SaleType {
    type Error = String;
    fn try_from(str: &str) -> Result<Self, String> {
        match str.to_lowercase().as_str() {
            "none" => Ok(Self::none()),
            "rent" => Ok(Self::rent()),
            "raffle" => Ok(Self::raffle()),
            "barter" => Ok(Self::barter()),
            "auction" => Ok(Self::auction()),
            _ => Err("Invalid value".to_string()),
        }
    }
}

impl SaleType {
    pub fn inner(&self) -> Inner {
        self.inner
    }
}

impl From<SaleType> for Inner {
    fn from(value: SaleType) -> Self {
        value.inner
    }
}

impl From<SaleType> for Vec<u8> {
    fn from(value: SaleType) -> Self {
        value.inner().into()
    }
}
