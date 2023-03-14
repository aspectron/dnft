use crate::{
    prelude::*,
    program::{MarketState, TokenCreateFinalArgs, TokenSaleSettingArgs},
};
use kaizen::result::Result;

pub struct Token;
declare_client!(program::Token, Token);

#[wasm_bindgen]
pub struct TokenMetaReference {
    inner: program::TokenMeta,
}

#[wasm_bindgen]
impl TokenMetaReference {
    pub fn version(&self) -> u32 {
        self.inner.get_version()
    }
    pub fn page(&self) -> u32 {
        self.inner.get_page()
    }
    pub fn mint(&self) -> Pubkey {
        self.inner.get_mint()
    }
    pub fn sale(&self) -> SaleReference {
        SaleReference {
            inner: self.inner.get_sale(),
        }
    }
    pub fn authority(&self) -> Pubkey {
        self.inner.get_authority()
    }
}
impl From<program::TokenMeta> for TokenMetaReference {
    fn from(inner: program::TokenMeta) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct ExchangeMechanicsReference {
    pub sale_type: String,
    pub coin: String,
    pub price: js_sys::BigInt,
}

impl ExchangeMechanicsReference {
    fn new(sale_type: &str, coin: &str, price: u64) -> Self {
        Self {
            sale_type: sale_type.to_string(),
            coin: coin.to_string(),
            price: js_sys::BigInt::from(price),
        }
    }
}

#[wasm_bindgen]
pub struct SaleReference {
    inner: program::token::Sale,
}

#[wasm_bindgen]
impl SaleReference {
    pub fn listed(&self) -> bool {
        self.inner.market_state == MarketState::Listed
    }

    pub fn sale_type(&self) -> String {
        self.inner.sale_type.into()
    }

    pub fn exchange_mechanics(&self) -> Option<ExchangeMechanicsReference> {
        self.exchange_mechanics_impl()
    }
}

impl SaleReference {
    pub fn exchange_mechanics_impl(&self) -> Option<ExchangeMechanicsReference> {
        let sale = match self.inner.exchange_mechanics {
            program::ExchangeMechanics::Sale(sale) => sale,
            _ => return None,
        };

        Some(match sale {
            program::exchange::Sale::Sol { price } => {
                ExchangeMechanicsReference::new("sale", "sol", price)
            }
            program::exchange::Sale::Spl { price, token } => {
                ExchangeMechanicsReference::new("sale-spl", &token.to_string(), price)
            }
        })
    }
}

impl Token {
    pub async fn create(
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
        let transaction = Transaction::new_with_accounts(
            format!("Creating token {token_pubkey}").as_str(),
            accounts,
            builder.try_into()?,
        );

        Ok(TransactionList::new(vec![transaction]))
    }
    pub async fn update_sale_setting(
        authority_pubkey: &Pubkey,
        token_pubkey: &Pubkey,
        args: &TokenSaleSettingArgs,
    ) -> Result<TransactionList> {
        if args.for_sale.is_none() && args.exchange_mechanics.is_none() && args.sale_type.is_none()
        {
            return Err("Nothing to update.".into());
        }
        let builder = client::Token::execution_context_for(program::Token::update_sale_setting)
            .with_authority(authority_pubkey)
            .with_handler_accounts(&[AccountMeta::new(*token_pubkey, false)])
            .with_instruction_data(&args.try_to_vec()?)
            .seal()?;

        let accounts = builder.gather_accounts(None, Some(token_pubkey))?;
        let transaction = Transaction::new_with_accounts(
            format!("Updating Token {token_pubkey}").as_str(),
            accounts,
            builder.try_into()?,
        );

        Ok(TransactionList::new(vec![transaction]))
    }

    pub async fn buy(authority_pubkey: &Pubkey, token_pubkey: &Pubkey) -> Result<TransactionList> {
        let token = load_container::<program::Token>(token_pubkey)
            .await?
            .ok_or_else(|| "Unable to load token container".to_string())?;
        let owner = token.meta.borrow().get_authority();
        log_trace!("owner:{}", owner.to_string());
        let builder = client::Token::execution_context_for(program::Token::buy)
            .with_authority(authority_pubkey)
            .with_handler_accounts(&[AccountMeta::new(*token_pubkey, false)])
            .with_system_program_account()
            .with_system_accounts(&[AccountMeta::new(owner, false)])
            .with_instruction_data(&Vec::new())
            .seal()?;

        let accounts = builder.gather_accounts(None, Some(token_pubkey))?;
        let transaction = Transaction::new_with_accounts(
            format!("Buy Token {token_pubkey}").as_str(),
            accounts,
            builder.try_into()?,
        );
        Ok(TransactionList::new(vec![transaction]))
    }
}

mod wasm {
    use super::{Token, TokenMetaReference};
    use crate::client::{Data, SaleType};
    use crate::prelude::*;
    use crate::program::{
        ExchangeMechanics, ForSale, MarketState, TokenCreateFinalArgs, TokenSaleSettingArgs,
    };
    use crate::wallet::Application;
    use kaizen::accounts::AccountReference;
    use kaizen::transport::api::*;
    use kaizen::utils::sol_to_lamports;
    use solana_program::account_info::IntoAccountInfo;
    use solana_sdk::account::Account;

    /// create a token for a specific mint
    #[wasm_bindgen(js_name = "createToken")]
    pub async fn create_token(
        mint: JsValue,
        for_sale: bool,
        sale_type: SaleType,
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
            } else {
                return Err(format!("Invalid field data at index : {index}").into());
            }
        }

        let for_sale = for_sale.into();
        log_trace!("create_token: data: {data:?}, for_sale:{for_sale:?}, sale_type: {sale_type:?}");

        let args = TokenCreateFinalArgs {
            for_sale,
            sale_type: sale_type.into(),
            exchange_mechanics: ExchangeMechanics::None, //TODO
            data,
        };
        let tx = Token::create(&authority, &mint, &args).await?;
        let token_account_pubkey = tx.target_account()?;
        tx.post().await?;
        Ok(to_value(&token_account_pubkey.to_string()).unwrap())
    }

    /// Returns a tokens for a specific mint
    #[wasm_bindgen(js_name = "getMarketTokens")]
    pub async fn get_market_tokens(mint: JsValue, page: u32) -> Result<JsValue, JsValue> {
        get_tokens(mint, page, Some(true), Some(true), &JsValue::UNDEFINED).await
    }

    /// Returns a tokens for a specific mint
    #[wasm_bindgen(js_name = "getMarketTokensByType")]
    pub async fn get_market_tokens_with_type(
        mint: JsValue,
        page: u32,
        sale_type: &JsValue,
    ) -> Result<JsValue, JsValue> {
        get_tokens(mint, page, Some(true), Some(true), sale_type).await
    }

    /// Returns all tokens for a specific mint
    #[wasm_bindgen(js_name = "getAllTokens")]
    pub async fn get_all_tokens(mint: JsValue, page: u32) -> Result<JsValue, JsValue> {
        get_tokens(mint, page, None, None, &JsValue::UNDEFINED).await
    }

    // Returns a tokens for a specific mint
    //#[wasm_bindgen(js_name = "getTokens")]
    async fn get_tokens(
        mint: JsValue,
        page: u32,
        market_state: Option<bool>,
        for_sale: Option<bool>,
        sale_type: &JsValue,
    ) -> Result<JsValue, JsValue> {
        let mint = Pubkey::from_value(&mint)?;
        let mut filters = vec![
            AccountFilter::MemcmpEncodeBase58(8, page.to_le_bytes().to_vec()),
            AccountFilter::MemcmpEncodedBase58(12, mint.to_string()),
            //AccountFilter::MemcmpEncodeBase58(40, vec![1]),
        ];

        let sale_type = workflow_wasm::abi::ref_from_abi_as_option!(SaleType, sale_type)?;
        if let Some(state) = market_state {
            let mut state_bytes: Vec<u8> = MarketState::from(state).into();
            if let Some(for_sale) = for_sale {
                let mut bytes = vec![];
                bytes.append(&mut state_bytes);
                bytes.append(&mut program::ForSale::from(for_sale).into());
                if let Some(sale_type) = sale_type {
                    bytes.append(&mut sale_type.into());
                }
                filters.push(AccountFilter::MemcmpEncodeBase58(44, bytes));
            } else {
                filters.push(AccountFilter::MemcmpEncodeBase58(44, state_bytes));
                if let Some(sale_type) = sale_type {
                    filters.push(AccountFilter::MemcmpEncodeBase58(46, sale_type.into()));
                }
            }
        } else if let Some(for_sale) = for_sale {
            let mut bytes: Vec<u8> = vec![];
            bytes.append(&mut program::ForSale::from(for_sale).into());
            if let Some(sale_type) = sale_type {
                bytes.append(&mut sale_type.into());
            }
            filters.push(AccountFilter::MemcmpEncodeBase58(45, bytes));
        } else if let Some(sale_type) = sale_type {
            filters.push(AccountFilter::MemcmpEncodeBase58(46, sale_type.into()));
        }

        //log_trace!("filters: {filters:#?}");

        let transport = Transport::global()?;
        let config = GetProgramAccountsConfig::new()
            .add_filters(filters)?
            .encoding(AccountEncoding::Base64)?
            .data_slice(AccountDataSliceConfig {
                offset: 0,
                length: 0,
            })?;

        let accounts = transport
            .get_program_accounts_with_config(&crate::program_id(), config)
            .await?;

        //log_trace!("accounts: {accounts:?}");

        let result = js_sys::Array::new();
        for (pubkey, _) in accounts {
            //log_trace!("get_tokens: pubkey:{pubkey}");
            if let Some(account) = transport.lookup(&pubkey).await? {
                result.push(&create_account_info(&pubkey, account)?.into());
            }
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
        Ok(create_account_info(&pubkey, account)?.into())
    }

    fn create_account_info(
        pubkey: &Pubkey,
        account: Arc<AccountDataReference>,
    ) -> Result<js_sys::Array, JsValue> {
        let account_data = account.clone_for_program()?;
        let mut account: Account = (&account_data).into();
        //let mut account_clone = account;
        let account_info = (pubkey, &mut account).into_account_info();
        let token = program::Token::try_load(&account_info)?;
        let meta = TokenMetaReference::from(**token.meta.borrow()).into();
        let data = js_sys::Array::new();
        if let Some(items) = token.data.load()? {
            //log_trace!("data: {items:?}");
            for item in items.into_iter() {
                data.push(&item.into());
            }
        }

        let item = js_sys::Array::new();
        item.push(&pubkey.to_string().into());
        item.push(&meta);
        item.push(&data);
        item.push(&AccountReference::from(&account_data).into());
        Ok(item)
    }

    /// Returns a token for a specific pubkey
    #[wasm_bindgen(js_name = "buyToken")]
    pub async fn buy_token(pubkey: JsValue) -> Result<(), JsValue> {
        Application::ensure_wallet().await?;
        let pubkey = Pubkey::from_value(&pubkey)?;
        let authority = Transport::global()?.get_authority_pubkey()?;

        let tx = Token::buy(&authority, &pubkey).await?;
        tx.post().await?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "updateTokenSaleSetting")]
    pub async fn update_token_sale_setting(
        pubkey: JsValue,
        for_sale: Option<bool>,
        price: Option<f64>,
    ) -> Result<JsValue, JsValue> {
        Application::ensure_wallet().await?;
        let pubkey = Pubkey::from_value(&pubkey)?;
        let authority = Transport::global()?.get_authority_pubkey()?;
        let for_sale: Option<ForSale> = for_sale.map(|v| v.into());

        let args = TokenSaleSettingArgs {
            for_sale,
            sale_type: None,
            exchange_mechanics: price
                .map(|price| ExchangeMechanics::sale(sol_to_lamports(price), None)),
        };
        let tx = Token::update_sale_setting(&authority, &pubkey, &args).await?;
        let ids = tx.ids()?;
        tx.post().await?;
        Ok(to_value(&ids)?)
    }
}
