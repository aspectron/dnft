use borsh::{BorshSerialize, BorshDeserialize};
use kaizen::wallet::foreign::*;
//use kaizen::wasm::adapters;
use wasm_bindgen::prelude::*;
use crate::client::result::Result;
use workflow_log::log_trace;
use workflow_store::Store;
use std::sync::{Arc,Mutex};
use workflow_wasm::prelude::*;

//static mut STORE_NAME:Option<String> = None;

static mut APPLICATION:Option<Application> = None;

#[derive(Clone)]
#[wasm_bindgen]
pub struct Application{
    store_name: String,
    data: Arc<Mutex<StoreData>>,

    /// holds references to [Callback](workflow_wasm::callback::Callback)
    connect_callbacks: CallbackMap,
}

#[wasm_bindgen]
impl Application{
    #[wasm_bindgen(constructor)]
    pub async fn new(store_name: &str)->Self{
        let data = match StoreData::get(store_name).await{
            Ok(data)=>data,
            Err(err)=>{
                panic!("Unable to create StoreData Object {err:?}")
            }
        };

        let app = Self{
            store_name: store_name.to_string(),
            data : Arc::new(Mutex::new(data)),
            connect_callbacks: CallbackMap::new()
        };

        unsafe{APPLICATION = Some(app.clone())};
        app
    }

    #[wasm_bindgen(js_name="onWalletConnect")]
    pub fn on_wallet_connect(&self, callback: js_sys::Function) -> Result<()>{
        let callback_ = callback!(move ||{
            let this = JsValue::null();
            let _ = callback.call0(&this);
        });

        self.connect_callbacks.retain(callback_)?;
        Ok(())
    }


    #[wasm_bindgen(js_name="connectWallet")]
    pub async fn connect_wallet(&self)->Result<()>{
        let adapters = Self::get_adapter_list().await?;
        for adapter in adapters{
            if adapter.detected{
                self.connect(adapter).await?;
            }
        }

        Ok(())
    }

    #[wasm_bindgen(js_name="checkWalletState")]
    pub async fn check_wallet_state(&self)->Result<()>{
        {
            let data = self.data.lock()?;
            if !data.wallet_auto_connect{
                return Ok(())
            }
        }
    
        self.connect_wallet().await?;

        Ok(())
    }

    async fn connect(&self, adapter: Adapter)->Result<()>{
        let wallet = Wallet::try_new()?;
        wallet.connect(Some(adapter)).await?;
        let mut data = self.data.lock()?;
        data.wallet_auto_connect = true;
        data.save(&self.store_name).await?;

        for (_id, callback) in self.connect_callbacks.inner().iter(){
            let _ = callback.get_fn().call0(&JsValue::null());
        }
    
        Ok(())
    }

    async fn get_adapter_list()->Result<Vec<Adapter>>{
        let wallet = Wallet::try_new()?;

        Ok(wallet.get_adapter_list().await?.unwrap_or(vec![]))
    }
    
    /*
    #[wasm_bindgen(js_name="setStoreName")]
    pub fn set_store_name(&self, store_name: &str)->Result<()>{
        *self.store_name.lock()? =  store_name.to_string();
        Ok(())
    }

    #[wasm_bindgen(js_name="getStoreName")]
    pub fn store_name(&self)->Result<String>{
        Ok(self.store_name.lock()?.clone())
    }
    */


}


#[derive(Debug, BorshDeserialize, BorshSerialize)]
struct StoreData{
    pub wallet_auto_connect: bool,

    //#[borsh_skip]
    //_store_name: String
}

impl StoreData{
    fn new()->Self{
        Self{
            wallet_auto_connect: false
        }
    }

    pub async fn get(store_name:&str)->Result<Self>{
        let store = Self::get_store(store_name);
        if !store.exists().await?{
            let data = StoreData::new();
            store.write(&data.try_to_vec()?).await?;
            return Ok(data)
        }
        let data = &store.read().await?;
        let data = StoreData::try_from_slice(data)?;

        log_trace!("store_data: {data:?}");

        Ok(data)
    }

    pub async fn save(&self, store_name:&str)->Result<()>{
        let store = Self::get_store(&store_name);
        store.write(&self.try_to_vec()?).await?;
        Ok(())
    }

    fn get_store(name: &str)->Store{
        let mut store = Store::new();
        store.with_generic(&format!("~/.{name}"));
        store
    }
}






