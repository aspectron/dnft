use crate::{prelude::*, program::TokenCreateFinalArgs};
use kaizen::result::Result;

pub struct Token;
declare_client!(program::Token, Token);

impl Token {
    pub async fn create<'channel>(
        authority_pubkey: &Pubkey,
        mint_pubkey: &Pubkey,
        args: &TokenCreateFinalArgs,
    ) -> Result<TransactionList> {
        let mint = reload_container::<program::Mint>(mint_pubkey)
            .await?
            .ok_or_else(|| "Unable to load mint container".to_string())?;

        let builder = client::Token::execution_context_for(program::Token::create_final)
            .with_authority(authority_pubkey)
            .with_collection_template(&mint.tokens)
            .await?
            .with_handler_accounts(&[AccountMeta::new(*mint.pubkey(), false)])
            .with_instruction_data(&args.try_to_vec()?)
            .seal()?;

        let token_pubkey = builder.collection_template_pubkey_at(0);
        let accounts = builder.gather_accounts(None, Some(&token_pubkey))?;
        // log_info!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ {:?}", accounts);
        let transaction = Transaction::new_with_accounts(
            format!("Creating token {token_pubkey}").as_str(),
            accounts,
            builder.try_into()?,
        );

        Ok(TransactionList::new(vec![transaction]))
    }
}

mod wasm {
    use super::Token;
    use crate::client::Data;
    use crate::prelude::*;
    use crate::program::TokenCreateFinalArgs;
    use kaizen::transport::api::*;
    use solana_program::account_info::IntoAccountInfo;
    use solana_sdk::account::Account;
    //use serde::de::Deserialize;

    /// create a token for a specific mint
    #[wasm_bindgen(js_name = "createToken")]
    pub async fn create_token(
        mint: JsValue,
        available: u8,
        values: js_sys::Array,
    ) -> Result<JsValue, JsValue> {
        let mint = Pubkey::from_value(&mint)?;
        let authority = Transport::global()?.get_authority_pubkey()?;
        let mut data = vec![];
        for index in 0..values.length() {
            let value = values.get(index);
            if !value.is_undefined() {
                let value: Data = value.as_ref().try_into().unwrap();
                data.push(value.into());
            }
        }

        log_trace!("create_token: data: {data:?}");

        let args = TokenCreateFinalArgs { available, data };
        let tx = Token::create(&authority, &mint, &args).await?;
        let token_account_pubkey = tx.target_account()?;
        tx.post().await?;
        Ok(to_value(&token_account_pubkey.to_string()).unwrap())
    }

    /// Returns a tokens for a specific mint
    #[wasm_bindgen(js_name = "getTokens")]
    pub async fn get_tokens(mint: JsValue) -> Result<JsValue, JsValue> {
        let mint = Pubkey::from_value(&mint)?;
        let transport = Transport::global()?;
        let config = GetProgramAccountsConfig::new()
            .add_filters(vec![
                AccountFilter::MemcmpEncodedBase58(8, mint.to_string()),
                AccountFilter::MemcmpEncodeBase58(40, vec![1]),
            ])?
            .encoding(AccountEncoding::Base64)?;

        let accounts = transport
            .get_program_accounts_with_config(&crate::program_id(), config)
            .await?;

        let result = js_sys::Array::new();
        for (pubkey, account) in accounts {
            result.push(&create_account_info(&pubkey, account)?.into());
        }

        Ok(result.into())
    }

    /// Returns a token for a specific pubkey
    #[wasm_bindgen(js_name = "getToken")]
    pub async fn get_token(pubkey: JsValue) -> Result<JsValue, JsValue> {
        let pubkey = Pubkey::from_value(&pubkey)?;
        let transport = Transport::global()?;

        let account = transport
            .lookup(&pubkey)
            .await?
            .ok_or(JsValue::from("Account not found"))?;
        let account: Account = (&account.clone_for_program()?).into();
        Ok(create_account_info(&pubkey, account)?.into())
    }

    fn create_account_info(pubkey: &Pubkey, account: Account) -> Result<js_sys::Array, JsValue> {
        let mut account_clone = account.clone();
        let account_info = (pubkey, &mut account_clone).into_account_info();
        let token = program::Token::try_load(&account_info)?;
        let data = js_sys::Array::new();
        if let Some(items) = token.data.load()? {
            //log_trace!("data: {items:?}");
            for item in items.into_iter() {
                data.push(&item.into());
            }
        }
        let item = js_sys::Array::new();
        item.push(&pubkey.to_string().into());
        item.push(&data);
        item.push(&to_value(&account).unwrap());
        Ok(item)
    }
}
