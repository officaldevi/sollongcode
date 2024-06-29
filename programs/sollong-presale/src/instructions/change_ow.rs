use anchor_lang::prelude::*;
use crate::errors::PreSaleError;
use crate::instructions::common::OwOperator;


pub fn change_owner(ctx: Context<OwOperator>, new_owner: Pubkey) -> Result<()> {
    let data = &mut ctx.accounts.data;

    if new_owner == ctx.accounts.user.key() {
        return Ok(());
    }

    require!(data.owner == ctx.accounts.user.key(), PreSaleError::OwError);

    data.owner = new_owner;

    msg!("Change owner from {} to {}", ctx.accounts.user.key(), new_owner);

    Ok(())
}