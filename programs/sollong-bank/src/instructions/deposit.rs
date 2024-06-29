use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke};
use anchor_lang::solana_program::system_instruction::{transfer};
use crate::errors::SollongError;
use crate::state::{UserData, Financial, Metadata};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = anchor_lang::solana_program::sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,
    #[account(mut, seeds = [b"metadata"], bump = metadata.bump)]
    pub metadata: Account<'info, Metadata>,
    #[account(mut, seeds = [b"user-data", user.key().as_ref(),], bump = user_data.bump)]
    pub user_data: Account<'info, UserData>,
    #[account(mut, seeds = [b"financial", & [user.key().as_ref().last().unwrap() % metadata.financial_index][..]], bump = financial.bump)]
    pub financial: Account<'info, Financial>,
    pub system_program: Program<'info, System>,
}

pub fn deposit(ctx: Context<Deposit>, pay_lamports: u64) -> Result<()> {
    let user_data = &mut ctx.accounts.user_data;
    let financial = &mut ctx.accounts.financial;

    require!(pay_lamports > 0, SollongError::ParametersError);

    let _current_timestamp = ctx.accounts.clock.unix_timestamp as u64;

    let ix = transfer(&ctx.accounts.user.key(), &financial.key(), pay_lamports);
    invoke(&ix, &[ctx.accounts.user.to_account_info(), financial.to_account_info()])?;

    user_data.deposit += pay_lamports;

    financial.total += pay_lamports;

    
    Ok(())
}