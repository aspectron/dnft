//!
//! Program root, managing all mint chains.
//!

use crate::prelude::*;

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
    pub fn create(ctx: &ContextReference) -> ProgramResult {
        let allocation_args = AccountAllocationArgs::new(AddressDomain::None);
        let mut root = Root::try_allocate(ctx, &allocation_args, 0)?;

        let mut meta = root.meta.borrow_mut();
        meta.set_version(1);
        root.mints.try_create()?;

        ctx.sync_rent(root.account(), &RentCollector::default())?;

        Ok(())
    }
}

declare_handlers!(Root::<'info, 'refs>, [Root::create]);
