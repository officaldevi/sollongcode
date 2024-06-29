use anchor_lang::prelude::*;
use crate::errors::SollongError;
use crate::instructions::{OwOp, OwOpInRound};

pub fn set_buy_share_limit(ctx: Context<OwOpInRound>, min_share: u32, max_share: u32) -> anchor_lang::Result<()> {
    let metadata = &ctx.accounts.metadata;
    let round_stock = &mut ctx.accounts.round_stock;

    require!(ctx.accounts.user.key() == metadata.owner, SollongError::OwError);

    round_stock.min_share = min_share;
    round_stock.max_share = max_share;

    Ok(())
}