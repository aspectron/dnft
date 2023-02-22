//!
//! Token - Instance of each DNFT token.
//!

use crate::prelude::*;
use program::Mint;

pub type DataVec = Vec<program::Data>;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenCreationArgs {
    pub data : Vec<Option<program::Data>>
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenUpdateArgs {
    pub data : Vec<(u16,program::Data)>
}

#[derive(Meta, Copy, Clone)]
#[repr(packed)]
pub struct TokenMeta {
    version: u32,
    // identity: Pubkey,
    mint: Pubkey,
}

#[container(Containers::Token)]
pub struct Token<'info, 'refs> {
    pub meta: RefCell<&'info mut TokenMeta>,
    pub store: SegmentStore<'info, 'refs>,
    // ---
    pub data: Serialized<'info, 'refs, DataVec>,
}

impl<'info, 'refs> Token<'info, 'refs> {
    pub fn create(ctx: &ContextReference) -> ProgramResult {
        let mut mint = Mint::try_load(&ctx.handler_accounts[0])?;

        let (tpl_data, tpl_account_info) = ctx.try_consume_collection_template_address_data()?;
        let token = mint.tokens.try_create_container::<Token>(
            ctx,
            tpl_data.seed,
            tpl_account_info,
            None,
        )?;

        let mut meta = token.meta.borrow_mut();
        meta.set_version(1);
        meta.set_mint(*mint.pubkey());
        drop(meta);

        let args = TokenCreationArgs::try_from_slice(ctx.instruction_data)?;
        token.update_data(&mint, &args)?;

        ctx.sync_rent(token.account(), &RentCollector::default())?;

        Ok(())
    }

    pub fn update_data(&self, mint: &Mint, _args : &TokenCreationArgs) -> ProgramResult {

        if let Some(_data_types) = mint.data_types.load()? {

        }

        Ok(())
    }
}

declare_handlers!(Token::<'info, 'refs>, [Token::create,]);
