use crate::prelude::*;
use kaizen::result::Result;
use program::Data;

pub struct StorageProviderSchema {
    /// name of the provider
    pub name: String,
    /// URL of the provider website
    pub provider_url: String,
    /// data access URL
    pub data_url: String,
}

impl StorageProviderSchema {
    pub fn to_data(&self) -> Result<Vec<Data>> {
        if self.name.is_empty() || self.provider_url.is_empty() || self.data_url.is_empty() {
            return Err(error!("Storage Provider Schema data is empty"));
        }

        Ok(vec![
            Data::String(self.name.clone()),
            Data::String(self.provider_url.clone()),
            Data::String(self.data_url.clone()),
        ])
    }
}
