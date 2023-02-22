use crate::program::DataType;
use crate::{prelude::*, program::MintCreationArgs};
use client::Field;
use kaizen::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintData {
    tokens: u64,
    root: Pubkey,
    frozen: bool,
    schema: Vec<Field>,
}

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

        let builder = client::Mint::execution_context_for(program::Mint::create)
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
        let builder = client::Mint::execution_context_for(program::Mint::update)
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

    pub async fn get_data(pubkey: Pubkey) -> Result<MintData> {
        let mint = load_container::<program::Mint>(&pubkey)
            .await?
            .ok_or_else(|| "Unable to load root container".to_string())?;

        let data_types = mint
            .data_types
            .load()?
            .unwrap_or(Box::<Vec<DataType>>::default());

        let names = mint.names.load()?.unwrap_or(Box::<Vec<String>>::default());

        let descriptions = mint
            .descriptions
            .load()?
            .unwrap_or(Box::<Vec<String>>::default());

        let mut schema = Vec::<Field>::new();
        if !names.is_empty() {
            for (idx, data_type) in data_types.iter().enumerate() {
                let name = names
                    .get(idx)
                    .ok_or_else(|| error!("invalid mint schema range (name)"))?;
                let description = descriptions
                    .get(idx)
                    .ok_or_else(|| error!("invalid mint schema range (description)"))?;
                schema.push(Field::new(*data_type, name.clone(), description.clone()));
            }
        }

        let meta = mint.meta.borrow();
        Ok(MintData {
            tokens: mint.tokens.len() as u64,
            schema,
            frozen: meta.get_frozen(),
            root: meta.get_root(),
        })
    }

    pub async fn get_token_pubkeys(pubkey: Pubkey, from: u64, to: u64) -> Result<Vec<Pubkey>> {
        let mint = load_container::<program::Mint>(&pubkey)
            .await?
            .ok_or_else(|| format!("Unable to load mint container {pubkey}"))?;

        let len = mint.tokens.len() as u64;

        if from > len {
            return Err(kaizen::error!(
                "invalid token sequence range from: {from} but length is: {len}"
            ));
        }

        let to = std::cmp::min(to, len);

        let list = (from..to)
            .map(|idx| mint.tokens.get_pubkey_at(&crate::program_id(), idx))
            .collect::<std::result::Result<Vec<Pubkey>, _>>()?;

        Ok(list)
    }
}

mod wasm {
    use super::Mint;
    use crate::client::Schema;
    use crate::prelude::*;

    /// Create mint information/schema
    #[wasm_bindgen(js_name = "createMint")]
    pub async fn create_mint_data(schema: Schema) -> Result<JsValue, JsValue> {
        let pubkey = Transport::global()?.get_authority_pubkey()?;
        let tx = Mint::create(&pubkey, &schema.into()).await?;
        let mint_account_pubkey = tx.target_account()?;
        tx.execute().await?;
        Ok(to_value(&mint_account_pubkey.to_string()).unwrap())
    }

    /// Returns general mint information
    #[wasm_bindgen(js_name = "getMintData")]
    pub async fn get_mint_data(pubkey: JsValue) -> Result<JsValue, JsValue> {
        Ok(to_value(&Mint::get_data(pubkey_from_value(pubkey)?).await?).unwrap())
    }

    /// Returns a range of token pubkeys for a specific mint
    #[wasm_bindgen(js_name = "getTokenPubkeys")]
    pub async fn get_token_pubkeys(pubkey: Pubkey, from: u64, to: u64) -> Result<JsValue, JsValue> {
        Ok(to_value(&Mint::get_token_pubkeys(pubkey, from, to).await?).unwrap())
    }
}
