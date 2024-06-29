use anchor_lang::prelude::*;
use crate::state::UserData;

#[derive(Accounts)]
pub struct InitUserData<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space=49, seeds = [b"user-data", user.key().as_ref()], bump)]
    pub data: Account<'info, UserData>,
    pub system_program: Program<'info, System>
}

pub fn init_user_data(ctx: Context<InitUserData>) -> anchor_lang::Result<()> {
    let user_data = &mut ctx.accounts.data;
    user_data.owner = ctx.accounts.user.key();
    user_data.bump = ctx.bumps.data;

    msg!("User data init: {} -> {}",  user_data.owner.key(), user_data.key());
    Ok(())
}