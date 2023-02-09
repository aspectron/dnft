//!
//! Token - Instance of each DNFT token.
//!

use crate::prelude::*;
use program::Mint;

pub type DataVec = Vec<program::Data>;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenCreationArgs {}

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

        ctx.sync_rent(token.account(), &RentCollector::default())?;

        Ok(())
    }
}

declare_handlers!(Token::<'info, 'refs>, [Token::create,]);
