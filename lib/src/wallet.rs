use kaizen::wallet::foreign::*;
//use kaizen::wasm::adapters;
use wasm_bindgen::prelude::*;
use kaizen::result::Result;

#[wasm_bindgen(js_name="connectWallet")]
pub async fn connect_wallet()->Result<()>{
    let wallet = Wallet::try_new()?;

    let adapters_list = wallet.get_adapter_list().await?;
    //let adapters = adapters()?;
    if let Some(mut list) = adapters_list{
        let a = list.remove(0);
        wallet.connect(Some(a)).await?;
    }

    Ok(())
}
