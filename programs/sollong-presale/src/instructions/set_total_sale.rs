use anchor_lang::prelude::*;
use crate::errors::PreSaleError;
use crate::instructions::OwOperator;

pub fn set_total_sale(ctx: Context<OwOperator>, total_sale: u64) -> anchor_lang::Result<()> {
    let data = &mut ctx.accounts.data;

    require!(ctx.accounts.user.key() == data.owner, PreSaleError::OwError);

    data.total_sale_amount = total_sale;
    data.remaining_sale_amount = total_sale;

    Ok(())
}