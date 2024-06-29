use anchor_lang::prelude::*;
use crate::state::{Financial, Metadata};
use crate::errors::SollongError;

#[derive(Accounts)]
pub struct CreateFinancial<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"metadata"], bump = metadata.bump)]
    pub metadata: Account<'info, Metadata>,
    #[account(init, payer = user, space=64, seeds = ["financial".as_bytes(), &[metadata.financial_index][..]], bump)]
    pub financial: Account<'info, Financial>,
    pub system_program: Program<'info, System>
}

pub fn create_financial_account(ctx: Context<CreateFinancial>) -> Result<()> {
    require!(ctx.accounts.user.key() == ctx.accounts.metadata.owner, SollongError::OwError);

    let financial = &mut ctx.accounts.financial;
    let metadata = &mut ctx.accounts.metadata;

    financial.index = metadata.financial_index;
    financial.bump = ctx.bumps.financial;
    financial.total = 0;

    msg!("Create New Financial: {} -> {}", financial.index, financial.key());

    metadata.financial_index += 1;

    Ok(())
}