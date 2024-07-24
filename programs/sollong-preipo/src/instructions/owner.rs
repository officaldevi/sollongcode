use anchor_lang::prelude::*;
use crate::state::{Metadata, RoundStock};

impl Metadata {
    pub fn check_owner(&self, user: &Pubkey, end_timestamp: u64) -> bool {
        return self.owner == *user  && self.owner_validate_time >= end_timestamp;
    }
}

#[derive(Accounts)]
pub struct OwOpInRound<'info> {
    #[account()]
    pub user: Signer<'info>,
    #[account(seeds = [b"metadata"], bump = metadata.bump)]
    pub metadata: Account<'info, Metadata>,
    #[account(mut, seeds = [&[round_stock.index][..], b"round-stock"], bump = round_stock.bump)]
    pub round_stock: Account<'info, RoundStock>,
    #[account(address = anchor_lang::solana_program::sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct OwOp<'info> {
    #[account()]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"metadata"], bump = metadata.bump)]
    pub metadata: Account<'info, Metadata>,
    #[account(address = anchor_lang::solana_program::sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,
}
