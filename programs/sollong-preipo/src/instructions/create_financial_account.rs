use anchor_lang::prelude::*;
use crate::state::{RoundStock, Financial, Metadata};
use crate::errors::SollongError;

#[derive(Accounts)]
pub struct CreateFinancial<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"metadata"], bump = metadata.bump)]
    pub metadata: Account<'info, Metadata>,
    #[account(mut, seeds = [&[round_stock.index][..], b"round-stock"], bump = round_stock.bump)]
    pub round_stock: Account<'info, RoundStock>,
    /// seeds = [round_index, "financial", financial_index]
    #[account(init, payer = user, space=64, seeds = [&[round_stock.index][..], "financial".as_bytes(), &[round_stock.financial_index][..]], bump)]
    pub financial: Account<'info, Financial>,
    pub system_program: Program<'info, System>
}

pub fn create_financial_account(ctx: Context<CreateFinancial>) -> Result<()> {
    require!(ctx.accounts.user.key() == ctx.accounts.metadata.owner, SollongError::OwError);

    let financial = &mut ctx.accounts.financial;
    let round = &mut ctx.accounts.round_stock;

    financial.round_index = round.index;
    financial.round_index = round.financial_index;
    financial.bump = ctx.bumps.financial;
    financial.total = 0;

    round.financial_index += 1;

    msg!("Create New Financial in Round({}): {} -> {}", round.index, round.financial_index, financial.key());
    Ok(())
}