use solana_program::program_error::ProgramError;

#[repr(u32)]
pub enum ErrorCode {
    DataIndex,
    DataTypeMismatch,
    // MintData,
    MintSchema,
    PartialTokenData,
    AccessDenied,
    TokenNotAvailableForSale,
}

impl From<ErrorCode> for ProgramError {
    fn from(err: ErrorCode) -> Self {
        ProgramError::Custom(err as u32)
    }
}

// impl From<ErrorCode> for u32 {
//     fn from(code: ErrorCode) -> u32 {
//         code as u32
//     }
// }

// impl std::ops::Deref for ErrorCode {
//     type Target = u32;
//     fn deref(&self) -> &Self::Target {
//         &(*self as u32)
//         // &self as &u32
//     }
// }
