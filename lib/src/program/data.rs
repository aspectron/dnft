use crate::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum Data {
    // None,
    Bool(bool),
    u8(u8),
    u16(u16),
    u32(u32),
    u64(u64),
    u128(u128),
    i8(i8),
    i16(i16),
    i32(i32),
    i64(i64),
    f32(f32),
    f64(f64),
    Flags32(u32),
    Flags64(u64),
    String(String),
    // Url(String),
    Url(Url),
    // PageUrl(String),
    // ImageUrl(String),
    // ResourceUrl(String),
    Date(u32), // <-- TODO
    Time(u32), // <-- TODO
    Geo(Geo),
    Pubkey(Pubkey),
    // Array(Vec<Data>),
    // Table(Vec<(Data, Data)>),
}

impl Data {
    pub fn get_data_type(&self) -> DataType {
        match self {
            // Data::None => DataType::None,
            Data::Bool(_) => DataType::Bool,
            Data::u8(_) => DataType::u8,
            Data::u16(_) => DataType::u16,
            Data::u32(_) => DataType::u32,
            Data::u64(_) => DataType::u64,
            Data::u128(_) => DataType::u128,
            Data::i8(_) => DataType::i8,
            Data::i16(_) => DataType::i16,
            Data::i32(_) => DataType::i32,
            Data::i64(_) => DataType::i64,
            Data::f32(_) => DataType::f32,
            Data::f64(_) => DataType::f64,
            Data::Flags32(_) => DataType::Flags32,
            Data::Flags64(_) => DataType::Flags64,
            Data::String(_) => DataType::String,
            Data::Url(url) => {
                match url{
                    Url::Image(_, _)=>DataType::ImageUrl,
                    Url::Page(_, _)=>DataType::PageUrl,
                    Url::StorageProviderAccess(_)=>DataType::StorageProviderUrl
                }
            },
            Data::Date(_) => DataType::Date,
            Data::Time(_) => DataType::Time,
            Data::Geo(_) => DataType::Geo,
            Data::Pubkey(_) => DataType::Pubkey,
            // Data::Array(_) => DataType::Array,
            // Data::Table(_) => DataType::Table,
        }
    }

    // pub fn is_none(&self) -> bool {
    //     matches!(self, Data::None)
    // }

    // pub fn is_some(&self) -> bool {
    //     !matches!(self, Data::None)
    // }
}

#[cfg(not(target_os = "solana"))]
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // use super::Data;
        match self {
            // Data::None => {
            //     write!(f, "None")
            // }
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
            Data::u128(v) => {
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
            // Data::PageUrl(v) => {
            //     write!(f, "{v}")
            // }
            // Data::ImageUrl(v) => {
            //     write!(f, "{v}")
            // }
            Data::Geo(v) => {
                write!(f, "{},{}", v.latitude, v.longitude)
            }
            // Data::Array(v) => {
            //     for item in v {
            //         writeln!(f, "\t{item}").unwrap();
            //     }
            //     Ok(())
            // }
            _ => write!(f, "{self:?}"),
        }
    }
}

#[cfg(not(target_os = "solana"))]
use wasm_bindgen::prelude::*;

#[cfg(not(target_os = "solana"))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UrlInfo{
    Image(String),
    Page(String),
    StorageProviderAccess(String)
}

#[cfg(not(target_os = "solana"))]
impl From<Data> for JsValue{
    fn from(data: Data)->JsValue{
        match data {
            Data::Bool(v) => v.into(),
            Data::u8(v) => v.into(),
            Data::u16(v) => v.into(),
            Data::u32(v) => v.into(),
            Data::u64(v) => v.into(),
            Data::u128(v) => v.into(),
            Data::i8(v) => v.into(),
            Data::i16(v) => v.into(),
            Data::i32(v) => v.into(),
            Data::i64(v) => v.into(),
            Data::f32(v) => v.into(),
            Data::f64(v) => v.into(),
            Data::Flags32(v) => v.into(),
            Data::Flags64(v) => v.into(),
            Data::String(v) => v.into(),
            Data::Url(v) => {
                match v{
                    Url::Image(base, url)=>{
                        to_value(&UrlInfo::Image(Url::build_url(base, &url))).unwrap()
                    },
                    Url::Page(base, url)=>{
                        to_value(&UrlInfo::Page(Url::build_url(base, &url))).unwrap()
                    },
                    Url::StorageProviderAccess(url)=>{
                        to_value(&UrlInfo::StorageProviderAccess(url)).unwrap()
                    },
                }
            },
            Data::Date(v) => v.into(),
            Data::Time(v) => v.into(),
            Data::Geo(v) => v.into(),
            Data::Pubkey(v) => v.into(),
        }
    }
}

u16_try_from! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Eq, PartialEq)]
    #[wasm_bindgen]
    #[repr(u16)]
    pub enum DataType {
        None,
        Bool,
        u8,
        u16,
        u32,
        u64,
        u128,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,
        String,
        Flags32,
        Flags64,
        PageUrl,
        ImageUrl,
        StorageProviderUrl,
        Date,
        Time,
        Geo,
        Pubkey,
        Array,
        Table,
        // TODO
        // Hash
    }
}

// #[derive(Debug, Clone, TryFromJsValue, BorshSerialize, BorshDeserialize)]
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
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

// #[derive(Debug, Clone, TryFromJsValue, BorshSerialize, BorshDeserialize)]
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
#[wasm_bindgen]
pub struct Hash256 {
    hash: [u8; 32],
}

cfg_if! {

 if #[cfg(not(target_os = "solana"))] {
        #[wasm_bindgen]
        impl Hash256 {
            #[wasm_bindgen(constructor)]
            pub fn new(text: String) -> Self {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(text);
                let hash = hasher.finalize();
                Self { hash: hash.into() }
            }

            pub fn check(&self) -> bool {
                true
            }
        }
    }
}
//     }
// }

// TODO
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Eq, PartialEq)]
pub enum UrlType {
    StorageProviderAccess,
    Page,
    Image,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Url {
    StorageProviderAccess(String),
    Page(u16, String),
    Image(u16, String),
}

#[cfg(not(target_os = "solana"))]
impl Url{
    pub fn parse_url(url: &str) ->(u16, String){
        let mut parts = url.split("/");
        parts.next();
        parts.next();
        if let Some(domain) = parts.next(){
            let url_path = url.replace(&format!("https://{domain}/"), "");
            match domain{
                "tinyurl.com"=>(1, url_path),
                _=>(0, url.to_string())
            }
        }else{
            return (0, url.to_string())
        }
    }

    pub fn build_url(base: u16, url: &str)->String{
        match base{
            1=>format!("https://tinyurl.com/{url}"),
            _=>url.to_string()
        }
    }

    pub fn image(url:&str)->Self{
        let (base, url) = Self::parse_url(url);
        Self::Image(base, url)
    }
    pub fn page(url:&str)->Self{
        let (base, url) = Self::parse_url(url);
        Self::Page(base, url)
    }
}

#[cfg(not(target_os = "solana"))]
impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // use super::Data;
        match self {
            Url::StorageProviderAccess(url) => write!(f, "{url}"),
            Url::Page(base, url) => write!(f, "{}", Self::build_url(*base, url)),
            Url::Image(base, url) => write!(f, "{}", Self::build_url(*base, url)),
        }
    }
}

#[cfg(not(target_os = "solana"))]
impl From<(UrlType, &str)> for Url {
    fn from((kind, url): (UrlType, &str)) -> Self {
        match kind {
            UrlType::StorageProviderAccess => Url::StorageProviderAccess(url.to_string()),
            UrlType::Page => {
                let (base, url) = Self::parse_url(url);
                Url::Page(base, url)
            },
            UrlType::Image =>{
                let (base, url) = Self::parse_url(url);
                Url::Image(base, url)
            },
        }
    }
}
