//!
//! Token - Instance of each DNFT token.
//!

use crate::prelude::*;
use kaizen::program_error_code;
use program::{Data, ExchangeMechanics, Mint, SaleType};
use solana_program::program::invoke;
use solana_program::system_instruction::transfer;

pub type DataVec = Vec<program::Data>;

#[derive(Default, Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenCreateFinalArgs {
    pub for_sale: ForSale,
    pub sale_type: SaleType,
    pub exchange_mechanics: ExchangeMechanics,
    pub data: Vec<Data>,
}

#[derive(Default, Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenSaleSettingArgs {
    pub for_sale: Option<ForSale>,
    pub sale_type: Option<SaleType>,
    pub exchange_mechanics: Option<ExchangeMechanics>,
}

#[derive(Default, Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenUpdateArgs {
    pub for_sale: Option<ForSale>,
    pub sale_type: Option<SaleType>,
    pub data: Vec<(u16, program::Data)>,
    // pub data: TokenDataArgs,
}

#[derive(Copy, Clone, Eq, PartialEq)]
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
#[repr(packed)]
pub struct Sale {
    pub market_state: MarketState,
    pub for_sale: ForSale,
    pub sale_type: program::SaleType,
    pub exchange_mechanics: ExchangeMechanics,
}

impl Sale {
    fn new(for_sale: ForSale, sale_type: SaleType, exchange_mechanics: ExchangeMechanics) -> Self {
        let market_state = if for_sale == ForSale::Yes || sale_type != SaleType::None {
            MarketState::Listed
        } else {
            MarketState::Unlisted
        };

        Self {
            market_state,
            for_sale,
            sale_type,
            exchange_mechanics,
        }
    }

    pub fn set_for_sale(&mut self, for_sale: ForSale) {
        let market_state = if for_sale == ForSale::Yes {
            // || self.sale_type != SaleType::None {
            MarketState::Listed
        } else {
            MarketState::Unlisted
        };
        self.market_state = market_state;
        self.for_sale = for_sale;
    }

    pub fn set_exchange_mechanics(&mut self, exchange_mechanics: ExchangeMechanics) {
        self.exchange_mechanics = exchange_mechanics;
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
    authority: Pubkey,
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
        }

        for (idx, data) in args.data.iter().enumerate() {
            if &data.get_data_type() != data_types.get(idx).unwrap() {
                return Err(program_error_code!(ErrorCode::MintSchema));
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
        meta.set_sale(Sale::new(
            args.for_sale,
            args.sale_type,
            args.exchange_mechanics,
        ));
        meta.set_reserved(0);
        meta.set_authority(*ctx.authority.key);
        drop(meta);

        token.data.store(&args.data)?;

        ctx.sync_rent(token.account(), &RentCollector::default())?;

        Ok(())
    }

    // pub fn exchange(ctx: &ContextReference) -> ProgramResult {

    //     // - LIST FOR SALE

    // }

    pub fn update_sale_setting(ctx: &ContextReference) -> ProgramResult {
        let token = Token::try_load(&ctx.handler_accounts[0])?;
        if &token.meta.borrow().get_authority() != ctx.authority.key {
            return Err(ProgramError::IllegalOwner);
        }
        let args = TokenSaleSettingArgs::try_from_slice(ctx.instruction_data)?;

        let mut meta = token.meta.borrow_mut();
        let mut sale = meta.sale;
        if let Some(for_sale) = args.for_sale {
            //meta.sale.set_for_sale(for_sale);<=== create error
            sale.set_for_sale(for_sale);
        }
        if let Some(exchange_mechanics) = args.exchange_mechanics {
            sale.set_exchange_mechanics(exchange_mechanics);
        }
        meta.set_sale(sale);

        Ok(())
    }

    pub fn buy(ctx: &ContextReference) -> ProgramResult {
        let token = Token::try_load(&ctx.handler_accounts[0])?;
        let authority = token.meta.borrow().get_authority();
        if &authority == ctx.authority.key {
            return Err(ProgramError::IllegalOwner);
        }

        let owner = ctx.system_accounts[1].clone();
        if &authority != owner.key {
            return Err(ProgramError::IllegalOwner);
        }

        let meta = token.meta.borrow_mut();
        let sale = meta.sale;
        if sale.market_state != MarketState::Listed {
            return Err(ErrorCode::TokenNotAvailableForSale.into());
        }

        let price = if let ExchangeMechanics::Sale(sale) = sale.exchange_mechanics {
            match sale {
                program::exchange::Sale::Sol { price } => price,
                program::exchange::Sale::Spl { price: _, token: _ } => {
                    return Err(ErrorCode::TokenNotAvailableForSale.into());
                }
            }
        } else {
            return Err(ErrorCode::TokenNotAvailableForSale.into());
        };

        let transfer_tokens_instruction = transfer(ctx.authority.key, owner.key, price);

        let required_accounts_for_transfer =
            [ctx.authority.clone(), owner, ctx.system_accounts[0].clone()];

        // Passing the TransactionInstruction to send
        invoke(
            &transfer_tokens_instruction,
            &required_accounts_for_transfer,
        )?;

        let token = Token::try_load(&ctx.handler_accounts[0])?;
        let mut meta = token.meta.borrow_mut();
        meta.set_authority(*ctx.authority.key);
        let mut sale = meta.sale;
        sale.set_for_sale(ForSale::No);
        meta.set_sale(sale);

        Ok(())
    }

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
    [
        Token::create_final,
        Token::update_sale_setting,
        Token::buy,
        Token::create_with_template
    ]
);
