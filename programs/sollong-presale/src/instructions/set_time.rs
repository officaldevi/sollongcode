use anchor_lang::prelude::*;
use crate::errors::PreSaleError;
use crate::instructions::OwOperator;

pub fn set_time(ctx: Context<OwOperator>, start_time: u64, end_time: u64) -> anchor_lang::Result<()> {
    let data = &mut ctx.accounts.data;

    require!(ctx.accounts.user.key() == data.owner, PreSaleError::OwError);

    data.start_timestamp = start_time;
    data.end_timestamp = end_time;
    Ok(())
}