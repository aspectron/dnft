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

mod wasm {
    use crate::prelude::*;
    use kaizen::error;
    // use serde_wasm_bindgen::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MintData {
        tokens: u64,
    }

    /// Returns general mint information
    #[wasm_bindgen]
    pub async fn get_mint_data(pubkey: Pubkey) -> Result<JsValue, JsValue> {
        let mint = load_container::<program::Mint>(&pubkey)
            .await?
            .ok_or_else(|| "Unable to load root container".to_string())?;

        let mint_data = MintData {
            tokens: mint.tokens.len() as u64,
            // TODO
        };

        Ok(to_value(&mint_data).unwrap())
    }

    /// Returns a range of token pubkeys for a specific mint
    #[wasm_bindgen]
    pub async fn get_token_pubkeys(pubkey: Pubkey, from: u64, to: u64) -> Result<JsValue, JsValue> {
        let mint = load_container::<program::Mint>(&pubkey)
            .await?
            .ok_or_else(|| "Unable to load root container".to_string())?;

        let len = mint.tokens.len() as u64;

        if from > len {
            return Err(
                error!("invalid token sequence range from: {from} but length is: {len}").into(),
            );
        }

        let to = std::cmp::min(to, len);

        let list = (from..to)
            .map(|idx| mint.tokens.get_pubkey_at(&crate::program_id(), idx))
            .collect::<std::result::Result<Vec<Pubkey>, _>>()?;

        Ok(to_value(&list).unwrap())
    }
}
