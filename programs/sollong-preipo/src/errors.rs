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
    #[msg("Merkle tree is ampty, Please set a value")]
    MerkleIsEmptyError,
    #[msg("Please call the 'buy' method for non-whitelisted purchases")]
    FunctionCallError,
    #[msg("Please call the 'buy_from_whitelist' method for whitelisted purchases")]
    FunctionCallError2,
    #[msg("The user does not have purchasing qualifications")]
    UserNotVerified,
    #[msg("This round of finance has been created")]
    FinancialCreateError,
    #[msg("The purchase amount must be greater than 0")]
    BuySharesError,
    #[msg("The user's purchase share does not meet the minimum share requirement")]
    UserBuySharesLimitMinimum,
    #[msg("The user's purchase share exceeds the maximum purchase share")]
    UserBuySharesLimitMaximum
}
