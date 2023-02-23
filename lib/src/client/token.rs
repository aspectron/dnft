use crate::{prelude::*, program::TokenCreationArgs};
use kaizen::result::Result;

pub struct Token;
declare_client!(program::Token, Token);

impl Token {
    pub async fn create<'channel>(
        authority_pubkey: &Pubkey,
        mint_pubkey: &Pubkey,
        args: &TokenCreationArgs,
    ) -> Result<TransactionList> {
        let mint = reload_container::<program::Mint>(mint_pubkey)
            .await?
            .ok_or_else(|| "Unable to load mint container".to_string())?;

        let builder = client::Token::execution_context_for(program::Token::create)
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
    use crate::prelude::*;
    use crate::program::TokenCreationArgs;

    /// Returns a range of mint pubkeys for a specific mint
    #[wasm_bindgen(js_name = "createToken")]
    pub async fn create_token(mint: JsValue) -> Result<JsValue, JsValue> {
        let mint = Pubkey::from_value(&mint)?;
        let authority = Transport::global()?.get_authority_pubkey()?;
        let args = TokenCreationArgs { data: Default::default() };
        let tx = Token::create(&authority, &mint, &args).await?;
        let token_account_pubkey = tx.target_account()?;
        tx.post().await?;
        Ok(to_value(&token_account_pubkey.to_string()).unwrap())
    }
}
