use anchor_lang::prelude::*;
use crate::errors::SollongError;
use crate::instructions::OwOpInRound;

pub fn set_buy_share_limit(ctx: Context<OwOpInRound>, min_share: u32, max_share: u32) -> anchor_lang::Result<()> {
    let round_stock = &mut ctx.accounts.round_stock;

    require!(ctx.accounts.metadata.check_owner(&ctx.accounts.user.key() , ctx.accounts.clock.unix_timestamp as u64), SollongError::OwError);

    round_stock.min_share = min_share;
    round_stock.max_share = max_share;

    Ok(())
}
