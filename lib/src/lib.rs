extern crate alloc;

pub mod exchange;

pub mod prelude;
pub mod test;

#[cfg(not(target_os = "solana"))]
pub mod client;
pub mod containers;
pub mod error;
pub mod program;

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
