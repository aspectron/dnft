use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, BorshSerialize, BorshDeserialize)]
#[repr(u8)]
pub enum SaleType {
    #[default]
    None = 0x0,
    //Sale    = 0x1,
    Rent = 0x2,
    Auction = 0x4,
    Barter = 0x8,
    Raffle = 0x10,
}

impl From<SaleType> for Vec<u8> {
    fn from(value: SaleType) -> Self {
        match value {
            SaleType::None => vec![0x0],
            //SaleType::Sale => vec![0x1],
            SaleType::Rent => vec![0x2],
            SaleType::Auction => vec![0x4],
            SaleType::Barter => vec![0x8],
            SaleType::Raffle => vec![0x10],
        }
    }
}
impl From<SaleType> for String {
    fn from(value: SaleType) -> Self {
        match value {
            SaleType::None => "None",
            //SaleType::Sale => "Sale",
            SaleType::Rent => "Rent",
            SaleType::Auction => "Auction",
            SaleType::Barter => "Barter",
            SaleType::Raffle => "Raffle",
        }.to_string()
    }
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
