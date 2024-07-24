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
    #[account(init, payer = user, space=19, seeds = [&[round_stock.index][..], "financial".as_bytes(), &[round_stock.financial_index][..]], bump)]
    pub financial: Account<'info, Financial>,
    #[account(address = anchor_lang::solana_program::sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>
}

pub fn create_financial_account(ctx: Context<CreateFinancial>) -> Result<()> {
    require!(ctx.accounts.metadata.check_owner(&ctx.accounts.user.key() , ctx.accounts.clock.unix_timestamp as u64), SollongError::OwError);

    let financial = &mut ctx.accounts.financial;
    let round = &mut ctx.accounts.round_stock;

    require!(round.financial_index == 0, SollongError::FinancialCreateError);

    financial.round_index = round.index;
    financial.bump = ctx.bumps.financial;
    financial.total = 0;

    round.financial_index += 1;

    msg!("Create New Financial in Round({}): {} -> {}", round.index, round.financial_index, financial.key());
    Ok(())
}
