use anchor_lang::prelude::*;
use crate::state::{Metadata, RoundStock};
use crate::errors::SollongError;

#[derive(Accounts)]
pub struct OwOpInRound<'info> {
    #[account()]
    pub user: Signer<'info>,
    #[account(seeds = [b"metadata"], bump = metadata.bump)]
    pub metadata: Account<'info, Metadata>,
    #[account(mut, seeds = [&[round_stock.index][..], b"round-stock"], bump = round_stock.bump)]
    pub round_stock: Account<'info, RoundStock>
}

#[derive(Accounts)]
pub struct OwOp<'info> {
    #[account()]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"metadata"], bump = metadata.bump)]
    pub metadata: Account<'info, Metadata>
}

pub fn change_owner(ctx: Context<OwOp>, new_owner: Pubkey) -> anchor_lang::Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    if new_owner == ctx.accounts.user.key() {
        return Ok(());
    }

    require!(metadata.owner == ctx.accounts.user.key(), SollongError::OwError);

    metadata.owner = new_owner;

    Ok(())
}