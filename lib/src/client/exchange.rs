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
