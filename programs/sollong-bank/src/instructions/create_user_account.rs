use anchor_lang::prelude::*;

use crate::state::UserData;

#[derive(Accounts)]
pub struct CreateUserData<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space=64, seeds = [b"user-data", user.key().as_ref()], bump)]
    pub user_data: Account<'info, UserData>,
    pub system_program: Program<'info, System>
}

pub fn create_user_account(ctx: Context<CreateUserData>) -> Result<()> {
    let user_data = &mut ctx.accounts.user_data;

    user_data.owner = ctx.accounts.user.key();
    user_data.bump = ctx.bumps.user_data;

    msg!("User data create: {} -> {}",  user_data.owner.key(), user_data.key());
    Ok(())
}

