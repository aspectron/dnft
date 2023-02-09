use crate::{prelude::*, program::TokenCreationArgs};
use kaizen::result::Result;

pub struct Token;
// declare_client!(program::Token, Token);

impl Token {
    pub async fn create<'channel>(
        authority_pubkey: &Pubkey,
        mint_pubkey: &Pubkey,
        args: &TokenCreationArgs,
    ) -> Result<TransactionList> {
        let mint = reload_container::<program::Mint>(&mint_pubkey)
            .await?
            .ok_or_else(|| "Unable to load mint container".to_string())?;

        let builder = client::Mint::execution_context_for(program::Mint::create_token)
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
}
