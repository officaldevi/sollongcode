use anchor_lang::prelude::*;

use crate::state::{RoundStock, UserData};

#[derive(Accounts)]
pub struct CreateUserData<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(seeds = [&[round_stock.index][..], b"round-stock"], bump = round_stock.bump)]
    pub round_stock: Account<'info, RoundStock>,
    
    /// seeds = [round_index, "user-data", user_key]
    #[account(init, payer = user, space=64, seeds = [&[round_stock.index][..], b"user-data", user.key().as_ref()], bump)]
    pub user_data: Account<'info, UserData>,
    pub system_program: Program<'info, System>
}

pub fn create_user_account(ctx: Context<CreateUserData>) -> anchor_lang::Result<()> {
    let user_data = &mut ctx.accounts.user_data;

    user_data.owner = ctx.accounts.user.key();
    user_data.round_index = ctx.accounts.round_stock.index;
    user_data.bump = ctx.bumps.user_data;

    msg!("User data create: {} -> {}",  user_data.owner.key(), user_data.key());
    Ok(())
}

