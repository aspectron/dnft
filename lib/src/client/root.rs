use crate::prelude::*;
use kaizen::result::Result;

pub struct Root;
declare_client!(program::Root, Root);

cfg_if! {
    if #[cfg(target_os = "solana")] {
        impl Root {
            #[inline]
            pub fn pubkey() -> Pubkey {
                find_root_pubkey()
            }
        }
    } else {
        // client-side: cache root pubkey
        static mut ROOT_PUBKEY: Option<Pubkey> = None;
        impl Root {
            pub fn pubkey() -> Pubkey {
                unsafe { ROOT_PUBKEY }.as_ref().cloned().unwrap_or_else(|| {
                    let pubkey = Root::find_root_pubkey();
                    unsafe { ROOT_PUBKEY = Some(pubkey) };
                    pubkey
                })
            }
        }
    }
}

impl Root {
    pub fn find_root_pubkey() -> Pubkey {
        let program_id = crate::program_id();
        let bytes = "root".as_bytes();
        let seed_suffix = bytes.to_vec();
        let seeds = vec![seed_suffix.as_ref()];
        let (address, _bump) = Pubkey::find_program_address(&seeds[..], &program_id);
        address
    }

    pub async fn create(
        authority_pubkey: &Pubkey,
        args: &program::RootCreationArgs,
    ) -> Result<TransactionList> {
        let builder = Root::execution_context_for(program::Root::create_root)
            .with_authority(authority_pubkey)
            .with_generic_account_templates_with_seeds(&[(AddressDomain::None, b"root")])
            .with_instruction_data(&args.try_to_vec()?)
            .seal()?;

        let root_pubkey = builder.generic_template_pubkey_at(0);
        let accounts = builder.gather_accounts(None, Some(&root_pubkey))?;

        let transaction = Transaction::new_with_accounts(
            format!("Creating root {root_pubkey}",).as_str(),
            accounts,
            builder.try_into()?,
        );

        Ok(TransactionList::new(vec![transaction]))
    }

    pub async fn get_mint_pubkeys(from: u64, to: u64) -> Result<Vec<Pubkey>> {
        let root = load_container::<program::Root>(&Root::pubkey())
            .await?
            .ok_or_else(|| "Unable to load root container".to_string())?;

        let len = root.mints.len() as u64;

        if from > len {
            return Err(kaizen::error!(
                "invalid token sequence range from: {from} but length is: {len}"
            ));
        }

        let to = std::cmp::min(to, len);

        let list = (from..to)
            .map(|idx| root.mints.get_pubkey_at(&crate::program_id(), idx))
            .collect::<std::result::Result<Vec<Pubkey>, _>>()?;

        Ok(list)
    }
}

mod wasm {
    use super::Root;
    use crate::prelude::*;

    /// Returns a range of token pubkeys for a specific mint
    #[wasm_bindgen(js_name = "getTokenPubkeys")]
    pub async fn get_token_pubkeys(from: u64, to: u64) -> Result<JsValue, JsValue> {
        Ok(to_value(&Root::get_mint_pubkeys(from, to).await?).unwrap())
    }
}
