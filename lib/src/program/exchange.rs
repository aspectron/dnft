use crate::prelude::*;

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
    Auction(Auction),
    Raffle(Raffle),
    Barter(Barter),
    Rent(Rent),
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
