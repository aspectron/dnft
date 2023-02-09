//!
//! Token Mint - managing all token chains
//! 

use crate::prelude::*;
use program::Token;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MintCreationArgs { }

// ~

#[derive(Meta, Copy, Clone)]
#[repr(packed)]
pub struct MintMeta {
    version: u32,
    root : Pubkey,
}

#[container(Containers::Mint)]
pub struct Mint<'info,'refs> {
    pub meta: RefCell<&'info mut MintMeta>,
    pub store: SegmentStore<'info,'refs>,
    // ---
    #[collection(seed(b"token"), container(program::Token))]
    pub tokens: PdaCollection<'info, 'refs>,

    pub schema : Serialized<'info,'refs,program::Schema>,
}


impl<'info, 'refs> Mint<'info, 'refs> {
    pub fn test(ctx: &ContextReference) -> ProgramResult {
        log_info!("TestInterface::test_handler CTX: {:#?}", ctx);

        Ok(())
    }

    pub fn init(&mut self, _ctx: &ContextReference) -> ProgramResult {
        self.meta.borrow_mut().set_version(1);
        self.tokens.try_create()?;

        // TODO - init mint data structure

        Ok(())
    }

    pub fn create_token(ctx: &ContextReference) -> ProgramResult {
        // msg!("TestInterface::test_handler CTX: {:#?}", ctx);

        let (tpl_data, tpl_account_info) = ctx.try_consume_collection_template_address_data()?;

        let mut mint = Mint::try_load(&ctx.handler_accounts[0])?;

        let mut token = mint.tokens
            .try_create_container::<Token>(
                ctx,
                tpl_data.seed,
                tpl_account_info,
                None,
            )?;

        token.init(ctx)?;

        ctx.sync_rent(token.account(), &RentCollector::default())?;

        Ok(())
    }
}

declare_handlers!(
    Mint::<'info, 'refs>,
    [
        Mint::test,
        Mint::create_token,
    ]
);
