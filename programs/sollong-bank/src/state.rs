use anchor_lang::prelude::*;

#[account]
pub struct Metadata {
    pub owner: Pubkey,
    pub skip_index: u8,
    pub financial_index: u8,
    pub bump: u8
}

#[account]
pub struct Financial {
    pub index: u8,
    pub total: u64,
    pub bump: u8
}
#[account]
pub struct UserData {
    pub owner: Pubkey,
    pub nonce: u64,
    pub deposit: u64,
    pub withdrawal: u64,
    pub bump: u8
}

#[account]
pub struct RoundStock {
    pub index: u8,
    pub whitelist_enabled: bool,
    pub merkel_root_hash: [u8;32],
    pub financial_index: u8,
    pub total_shares: u32,
    pub remaining_shares: u32,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub min_share: u32,
    pub max_share: u32,
    pub per_price: u64,
    pub bump: u8
}

