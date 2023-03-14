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
