use anchor_lang::prelude::*;
use crate::state::{Metadata, UserData};

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space=42, seeds = [b"metadata"], bump)]
    pub metadata: Account<'info, Metadata>,
    pub system_program: Program<'info, System>
}

pub fn init(ctx: Context<Init>) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    metadata.bump = ctx.bumps.metadata;
    metadata.owner = ctx.accounts.user.key();

    msg!("Sollong Program Init ...");

    Ok(())
}