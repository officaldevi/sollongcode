use anchor_lang::prelude::*;
use crate::errors::SollongError;
use crate::state::{Metadata, RoundStock};

#[derive(Accounts)]
pub struct NewRound<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"metadata"], bump = metadata.bump)]
    pub metadata: Account<'info, Metadata>,
    #[account(init, payer = user, space=88, seeds = [&[metadata.round_index][..], b"round-stock"], bump)]
    pub round_stock: Account<'info, RoundStock>,
    #[account(address = anchor_lang::solana_program::sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>
}

pub fn new_round(ctx: Context<NewRound>, total_shares: u32, min_share: u32, max_share: u32, per_price: u64) -> Result<()> {
    require!(ctx.accounts.metadata.check_owner(&ctx.accounts.user.key() , ctx.accounts.clock.unix_timestamp as u64), SollongError::OwError);

    let round = &mut ctx.accounts.round_stock;
    let metadata = &mut ctx.accounts.metadata;

    round.index = metadata.round_index;
    round.total_shares = total_shares;
    round.remaining_shares = round.total_shares;
    round.min_share = min_share;
    round.max_share = max_share;
    round.per_price = per_price;
    round.bump = ctx.bumps.round_stock;

    metadata.round_index += 1;

    msg!("New Round Start -> {}", ctx.accounts.metadata.round_index);

    Ok(())
}
