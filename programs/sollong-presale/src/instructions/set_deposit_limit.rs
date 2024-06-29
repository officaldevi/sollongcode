use anchor_lang::prelude::*;
use crate::errors::PreSaleError;
use crate::instructions::OwOperator;

pub fn set_deposit_limit(ctx: Context<OwOperator>, minimum_deposit: u64) -> anchor_lang::Result<()> {
    let data = &mut ctx.accounts.data;

    require!(ctx.accounts.user.key() == data.owner, PreSaleError::OwError);

    data.minimum_deposit = minimum_deposit;

    Ok(())
}