//!
//! Program root, managing all mint chains.
//!

use crate::prelude::*;
use program::{Mint, MintCreationArgs};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct RootCreationArgs {}

// ~

#[derive(Meta, Copy, Clone)]
#[repr(packed)]
pub struct RootMeta {
    version: u32,
}

#[container(Containers::Root)]
pub struct Root<'info, 'refs> {
    pub meta: RefCell<&'info mut RootMeta>,
    pub store: SegmentStore<'info, 'refs>,
    // ---
    #[collection(seed(b"mint"), container(program::Mint))]
    pub mints: PdaCollection<'info, 'refs>,
}

impl<'info, 'refs> Root<'info, 'refs> {
    pub fn test(ctx: &ContextReference) -> ProgramResult {
        log_info!("TestInterface::test_handler CTX: {:#?}", ctx);

        Ok(())
    }

    pub fn create_root(ctx: &ContextReference) -> ProgramResult {
        let allocation_args = AccountAllocationArgs::new(AddressDomain::None);
        let mut root = Root::try_allocate(ctx, &allocation_args, 0)?;

        let mut meta = root.meta.borrow_mut();
        meta.set_version(1);
        root.mints.try_create()?;

        ctx.sync_rent(root.account(), &RentCollector::default())?;

        Ok(())
    }

    pub fn create_mint(ctx: &ContextReference) -> ProgramResult {
        // msg!("TestInterface::test_handler CTX: {:#?}", ctx);

        let args = MintCreationArgs::try_from_slice(ctx.instruction_data)?;

        let (tpl_data, tpl_account_info) = ctx.try_consume_collection_template_address_data()?;

        let mut root = Root::try_load(&ctx.handler_accounts[0])?;

        let mut mint =
            root.mints
                .try_create_container::<Mint>(ctx, tpl_data.seed, tpl_account_info, None)?;

        mint.init(ctx, &args)?;

        ctx.sync_rent(mint.account(), &RentCollector::default())?;

        Ok(())
    }
}

declare_handlers!(
    Root::<'info, 'refs>,
    [Root::test, Root::create_root, Root::create_mint,]
);

// #[cfg(not(target_os = "solana"))]
// pub mod client {
//     use super::*;

// }
