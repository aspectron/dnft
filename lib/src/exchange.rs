use crate::prelude::*;

pub enum Sale {
    Sol { price: u64 },
    Spl { token: Pubkey, price: u64 },
}

pub struct Auction {}
pub struct Raffle {}
pub struct Barter {}
pub struct Rent {}

pub enum Mechanics {
    Sale(Sale),
    Auction(Auction),
    Raffle(Raffle),
    Barter(Barter),
    Rent(Rent),
}
pub struct Rules {}
