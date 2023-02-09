//!
//! Token - Instance of each DNFT token.
//! 

use crate::prelude::*;

pub type DataVec = Vec<program::Data>;


#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenCreationArgs { }

#[derive(Meta, Copy, Clone)]
#[repr(packed)]
pub struct TokenMeta {
    version: u32,
    // identity: Pubkey,
    mint : Pubkey,
}

#[container(Containers::Token)]
pub struct Token<'info,'refs> {
    pub meta: RefCell<&'info mut TokenMeta>,
    pub store: SegmentStore<'info,'refs>,
    // ---
    pub data: Serialized<'info,'refs,DataVec>,

}


impl<'info, 'refs> Token<'info, 'refs> {
    pub fn test(ctx: &ContextReference) -> ProgramResult {
        log_info!("TestInterface::test_handler CTX: {:#?}", ctx);

        Ok(())
    }

    pub fn init(&mut self, _ctx: &ContextReference) -> ProgramResult {
        self.meta.borrow_mut().set_version(1);

        // TODO - init token data structure

        Ok(())
    }

}

