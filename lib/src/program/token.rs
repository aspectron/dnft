//!
//! Token - Instance of each DNFT token.
//!

use crate::prelude::*;
use kaizen::program_error_code;
use program::{Data, Mint, SaleType};

pub type DataVec = Vec<program::Data>;

#[derive(Default, Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenCreateFinalArgs {
    pub for_sale: ForSale,
    pub sale_type: SaleType,
    pub data: Vec<Data>,
}

#[derive(Default, Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenUpdateArgs {
    pub for_sale: Option<ForSale>,
    pub sale_type: Option<SaleType>,
    pub data: Vec<(u16, program::Data)>,
    // pub data: TokenDataArgs,
}

#[derive(Copy, Clone)]
pub enum MarketState {
    Unlisted = 0x0,
    Listed = 0x1,
}

impl From<MarketState> for Vec<u8> {
    fn from(value: MarketState) -> Self {
        match value {
            MarketState::Unlisted => vec![0x0],
            MarketState::Listed => vec![0x1],
        }
    }
}
impl From<bool> for MarketState {
    fn from(value: bool) -> Self {
        if value {
            MarketState::Listed
        } else {
            MarketState::Unlisted
        }
    }
}

#[derive(Copy, Clone)]
pub struct Sale {
    pub market_state: MarketState,
    pub for_sale: ForSale,
    pub sale_type: program::SaleType,
}

impl Sale {
    fn new(for_sale: ForSale, sale_type: SaleType) -> Self {
        let mut market_state = MarketState::Unlisted;
        if for_sale == ForSale::Yes || sale_type != SaleType::None {
            market_state = MarketState::Listed;
        }

        Self {
            market_state,
            for_sale,
            sale_type,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, BorshSerialize, BorshDeserialize)]
pub enum ForSale {
    #[default]
    No = 0x0,
    Yes = 0x1,
}
impl From<ForSale> for Vec<u8> {
    fn from(value: ForSale) -> Self {
        match value {
            ForSale::No => vec![0x0],
            ForSale::Yes => vec![0x1],
        }
    }
}
impl From<bool> for ForSale {
    fn from(value: bool) -> Self {
        if value {
            ForSale::Yes
        } else {
            ForSale::No
        }
    }
}

#[derive(Meta, Copy, Clone)]
#[repr(packed)]
pub struct TokenMeta {
    version: u32,
    page: u32, // TODO - derive from sequence
    mint: Pubkey,
    sale: Sale,
    reserved: u8,
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
        let page = (mint.tokens.len() / 5) as u32;

        //log_trace!("create_final#### page:{}", page);
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
        meta.set_page(page);
        meta.set_sale(Sale::new(args.for_sale, args.sale_type));
        meta.set_reserved(0);
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
