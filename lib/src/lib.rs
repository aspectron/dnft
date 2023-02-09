extern crate alloc;

pub mod exchange;

pub mod prelude;
pub mod test;

#[cfg(not(target_os = "solana"))]
pub mod client;
pub mod program;
pub mod error;
pub mod containers;

use crate::prelude::*;

declare_program!(
    "dnft",
    "5UAQGzYRWKEgdbpZCqoUjKDKiWpNbHeataWknRpvswEH",
    [
        program::Root,
        program::Mint,
        // program::Token,
    ]
);
