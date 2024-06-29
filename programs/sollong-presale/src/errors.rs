use anchor_lang::error_code;

#[error_code]
pub enum PreSaleError {
    #[msg("Must meet the minimum deposit")]
    InsufficientDeposit,
    #[msg("Not in the current participation time period.")]
    TimeError,
    #[msg("Only the owner can call")]
    OwError,
}