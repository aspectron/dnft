use crate::{prelude::*, program::MintCreationArgs};
use kaizen::result::Result;

pub struct Mint;
declare_client!(program::Mint, Mint);

impl Mint {
    pub async fn create(
        authority_pubkey: &Pubkey,
        args: &MintCreationArgs,
    ) -> Result<TransactionList> {
        let root = reload_container::<program::Root>(&client::Root::pubkey())
            .await?
            .ok_or_else(|| "Unable to load root container".to_string())?;

        let builder = client::Root::execution_context_for(program::Root::create_mint)
            .with_authority(authority_pubkey)
            .with_collection_template(&root.mints)
            .await?
            .with_handler_accounts(&[AccountMeta::new(*root.pubkey(), false)])
            .with_instruction_data(&args.try_to_vec()?)
            .seal()?;

        let mint_pubkey = builder.collection_template_pubkey_at(0);
        let accounts = builder.gather_accounts(None, Some(&mint_pubkey))?;

        let transaction = Transaction::new_with_accounts(
            format!("Creating mint {mint_pubkey}").as_str(),
            accounts,
            builder.try_into()?,
        );

        Ok(TransactionList::new(vec![transaction]))
    }

    pub async fn update(
        authority_pubkey: &Pubkey,
        args: &MintCreationArgs,
        mint_pubkey: &Pubkey,
    ) -> Result<TransactionList> {
        let builder = client::Root::execution_context_for(program::Root::create_mint)
            .with_authority(authority_pubkey)
            .with_handler_accounts(&[AccountMeta::new(*mint_pubkey, false)])
            .with_instruction_data(&args.try_to_vec()?)
            .seal()?;

        let accounts = builder.gather_accounts(None, Some(mint_pubkey))?;
        let transaction = Transaction::new_with_accounts(
            format!("Creating mint {mint_pubkey}").as_str(),
            accounts,
            builder.try_into()?,
        );

        Ok(TransactionList::new(vec![transaction]))
    }
}
