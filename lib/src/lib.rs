extern crate alloc;

pub mod prelude;
pub mod test;

#[cfg(not(target_os = "solana"))]
pub mod client;
pub mod containers;
pub mod error;
pub mod program;

#[cfg(not(target_os = "solana"))]
pub mod wallet;

use crate::prelude::*;

declare_program!(
    "dnft",
    "5UAQGzYRWKEgdbpZCqoUjKDKiWpNbHeataWknRpvswEH",
    [program::Root, program::Mint, program::Token,]
);

cfg_if! {
    if #[cfg(not(target_os = "solana"))] {
        #[wasm_bindgen]
        pub fn dnft_program_id() -> Pubkey {
            program_id()
        }
    }
}
