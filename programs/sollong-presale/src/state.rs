use anchor_lang::prelude::*;

#[account]
pub struct PreSaleData {
    pub owner: Pubkey,
    pub total_sale_amount: u64,
    pub remaining_sale_amount: u64,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub minimum_deposit: u64,
    pub bump: u8
}

#[account]
pub struct UserData {
    pub owner: Pubkey,
    pub deposit: u64,
    pub bump: u8
}
