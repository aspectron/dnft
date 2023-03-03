//!
//! Token - Instance of each DNFT token.
//!

use crate::prelude::*;
use kaizen::program_error_code;
use program::{Data, Mint};

pub type DataVec = Vec<program::Data>;

#[derive(Default, Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenCreateFinalArgs {
    pub available: u8,
    pub data: Vec<Data>,
}

#[derive(Default, Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenUpdateArgs {
    pub available: Option<u8>,
    pub data: Vec<(u16, program::Data)>,
    // pub data: TokenDataArgs,
}

// #[derive(Default, Clone, Debug, BorshSerialize, BorshDeserialize)]
// pub struct TokenDataArgs {
//     pub data: Vec<(u16, program::Data)>,
// }

#[derive(Meta, Copy, Clone)]
#[repr(packed)]
pub struct TokenMeta {
    version: u32,
    // identity: Pubkey,
    page : u32, // TODO - derive from sequence
    mint: Pubkey,
    available: u8, // TODO - remov this
    sale_type : program::SaleType,
}

#[container(Containers::Token)]
pub struct Token<'info, 'refs> {
    pub meta: RefCell<&'info mut TokenMeta>,
    pub store: SegmentStore<'info, 'refs>,
    // ---
    pub data: Serialized<'info, 'refs, Vec<program::Data>>,
    // pub exchange: Serialized<'info, 'refs, program::Rules>,
}

impl<'info, 'refs> Token<'info, 'refs> {
    pub fn create_with_template(_ctx: &ContextReference) -> ProgramResult {
        todo!();
    }

    pub fn create_final(ctx: &ContextReference) -> ProgramResult {
        let mut mint = Mint::try_load(&ctx.handler_accounts[0])?;

        let args = Box::new(TokenCreateFinalArgs::try_from_slice(ctx.instruction_data)?);

        let data_types = mint
            .data_types
            .load()?
            .ok_or::<ProgramError>(program_error_code!(ErrorCode::MintSchema))?;

        if data_types.len() != args.data.len() {
            return Err(program_error_code!(ErrorCode::MintSchema));
        } else {
            for (idx, data) in args.data.iter().enumerate() {
                if &data.get_data_type() != data_types.get(idx).unwrap() {
                    return Err(program_error_code!(ErrorCode::MintSchema));
                }
            }
        }

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
        meta.set_available(args.available);
        drop(meta);

        token.data.store(&args.data)?;

        ctx.sync_rent(token.account(), &RentCollector::default())?;

        Ok(())
    }

    // pub fn exchange(ctx: &ContextReference) -> ProgramResult {

    //     // - LIST FOR SALE

    // }

    // pub fn update(ctx: &ContextReference) -> ProgramResult {
    //     let mint = Mint::try_load(&ctx.handler_accounts[0])?;
    //     let mut token = Token::try_load(&ctx.handler_accounts[1])?;
    //     let args = TokenUpdateArgs::try_from_slice(ctx.instruction_data)?;
    //     token.update_data(&mint, &args.data)?;
    //     Ok(())
    // }

    // pub fn update_data(&mut self, mint: &Mint, args: &TokenUpdateArgs) -> ProgramResult {
    //     // let data_types = mint.data_types.load()?.ok_or(ProgramError::Custom(ErrorCode::MintData.into()))?;
    //     let data_types = mint
    //         .data_types
    //         .load()?
    //         .ok_or::<ProgramError>(program_error_code!(ErrorCode::MintData))?;
    //     let mut token_data = self.data.load_or_default()?;
    //     if token_data.is_empty() {
    //         token_data.resize(args.data.len(), Data::None);
    //     }

    //     for (idx, incoming_data) in args.data.iter() {
    //         let idx = *idx as usize;
    //         let src_dt = data_types
    //             .get(idx)
    //             .ok_or::<ProgramError>(ErrorCode::DataIndex.into())?;
    //         let dst_dt = incoming_data.get_data_type();
    //         if src_dt != &dst_dt {
    //             return Err(program_error_code!(ErrorCode::DataTypeMismatch));
    //         }

    //         *token_data.get_mut(idx).unwrap() = incoming_data.clone();
    //     }

    //     self.data.store(&token_data)?;

    //     // TODO - seal token

    //     Ok(())
    // }

    // pub fn check_data(&self, data: &[Data]) -> ProgramResult {
    //     for item in data.iter() {
    //         if item.is_none() {
    //             return Err(program_error_code!(ErrorCode::PartialTokenData));
    //         }
    //     }
    //     Ok(())
    // }
}

declare_handlers!(
    Token::<'info, 'refs>,
    [Token::create_final, Token::create_with_template]
);
