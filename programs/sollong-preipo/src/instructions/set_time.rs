use anchor_lang::prelude::*;

use crate::errors::SollongError;
use crate::instructions::OwOpInRound;

pub fn set_time(ctx: Context<OwOpInRound>, start_time: u64, end_time: u64) -> anchor_lang::Result<()> {
    let round_stock = &mut ctx.accounts.round_stock;
    let metadata = &ctx.accounts.metadata;

    require!(ctx.accounts.user.key() == metadata.owner, SollongError::OwError);

    round_stock.start_timestamp = start_time;
    round_stock.end_timestamp = end_time;

    Ok(())
}
