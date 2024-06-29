use anchor_lang::error_code;

#[error_code]
pub enum SollongError {
    #[msg("The available shares for purchase are insufficient or exceed the limit")]
    InsufficientShares,
    #[msg("The available lamports for withdraw are insufficient or exceed the limit")]
    InsufficientBalance,
    #[msg("Not in the current participation time period.")]
    TimeError,
    #[msg("Only the owner can call")]
    OwError,
    #[msg("Merkle Tree Error")]
    MerkleError,
    #[msg("Please call the 'buy' method for non-whitelisted purchases")]
    FunctionCallError,
    #[msg("The user does not have purchasing qualifications")]
    UserNotVerified,
    #[msg("Parameter error")]
    ParametersError
}