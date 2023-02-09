//!
//! Token Mint - managing all token chains
//!

use crate::prelude::*;
use kaizen::error::program_error_code;
use program::DataType;
use program::Token;

#[derive(Clone, Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct MintCreationArgs {
    pub data_types: Option<Vec<DataType>>,
    pub names: Option<Vec<String>>,
    pub descriptions: Option<Vec<String>>,
}

// ~

#[derive(Meta, Copy, Clone)]
#[repr(packed)]
pub struct MintMeta {
    version: u32,
    root: Pubkey,
    frozen: bool,
}

#[container(Containers::Mint)]
pub struct Mint<'info, 'refs> {
    pub meta: RefCell<&'info mut MintMeta>,
    pub store: SegmentStore<'info, 'refs>,
    // ---
    #[collection(seed(b"token"), container(program::Token))]
    pub tokens: PdaCollection<'info, 'refs>,
    // ---
    pub data_types: Serialized<'info, 'refs, Vec<DataType>>,
    pub names: Serialized<'info, 'refs, Vec<String>>,
    pub descriptions: Serialized<'info, 'refs, Vec<String>>,
}

impl<'info, 'refs> Mint<'info, 'refs> {
    pub fn init(&mut self, _ctx: &ContextReference, args: &MintCreationArgs) -> ProgramResult {
        // set container version
        let mut meta = self.meta.borrow_mut();
        meta.set_version(1);
        meta.set_frozen(false);
        drop(meta);
        // init token chain
        self.tokens.try_create()?;
        // update mint settings during init stage (meant to be schema only)
        self.update_data(args)?;

        Ok(())
    }

    pub fn update(ctx: &ContextReference) -> ProgramResult {
        let mut mint = Mint::try_load(&ctx.handler_accounts[0])?;
        if mint.meta.borrow().frozen {
            return Err(program_error_code!(ErrorCode::AccessDenied));
        }
        let args = MintCreationArgs::try_from_slice(ctx.instruction_data)?;
        mint.update_data(&args)?;
        Ok(())
    }

    pub fn freeze(ctx: &ContextReference) -> ProgramResult {
        let mint = Mint::try_load(&ctx.handler_accounts[0])?;
        if mint.meta.borrow().frozen {
            return Err(program_error_code!(ErrorCode::AccessDenied));
        }
        mint.meta.borrow_mut().set_frozen(true);
        Ok(())
    }

    pub fn update_data(&mut self, args: &MintCreationArgs) -> ProgramResult {
        log_info!("Mint::update CTX: {:#?}", args);

        if let Some(data_types) = &args.data_types {
            self.data_types.store(data_types)?;
        }

        if let Some(names) = &args.names {
            self.names.store(names)?;
        }

        if let Some(descriptions) = &args.descriptions {
            self.descriptions.store(descriptions)?;
        }

        Ok(())
    }

    pub fn create_token(ctx: &ContextReference) -> ProgramResult {
        let (tpl_data, tpl_account_info) = ctx.try_consume_collection_template_address_data()?;

        let mut mint = Mint::try_load(&ctx.handler_accounts[0])?;

        let mut token = mint.tokens.try_create_container::<Token>(
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
        // Mint::update,
        Mint::create_token,
    ]
);
