use crate::prelude::*;

#[derive(Clone, BorshSerialize, BorshDeserialize)]
#[repr(u16)]
pub enum SaleType {
    NoSale = 0,
    Sale = 0x0101,
    Rent = 0x0102,
    Auction = 0x0103,
    Barter = 0x0104,
    Raffle = 0x0105,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub enum Sale {
    Sol { price: u64 },
    Spl { token: Pubkey, price: u64 },
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct Auction {}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct Raffle {}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct Barter {}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct Rent {}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub enum ExchangeMechanics {
    Sale(Sale),
    Rent(Rent),
    Auction(Auction),
    Barter(Barter),
    Raffle(Raffle),
}

// #[derive(Clone, BorshSerialize, BorshDeserialize)]
// pub struct Rules {
//     pub mechanics: ExchangeMechanics,
// }

// #[derive(Clone, BorshSerialize, BorshDeserialize)]
// pub enum ExchangeOp {
//     List,
//     Unlist,

// }
