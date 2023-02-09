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
}
