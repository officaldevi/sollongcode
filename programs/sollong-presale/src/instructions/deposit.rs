use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::errors::PreSaleError;
use crate::state::{PreSaleData, UserData};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = anchor_lang::solana_program::sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,
    #[account(mut, seeds = [b"presale-data"], bump = data.bump)]
    pub data: Account<'info, PreSaleData>,
    #[account(mut, seeds = [b"user-data", user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, UserData>,
    pub system_program: Program<'info, System>
}




pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let min_amount = 10u64.pow(7);

    let data = &mut ctx.accounts.data;
    let user_data = &mut ctx.accounts.user_data;

    let total_deposit = amount + user_data.deposit;

    require!(total_deposit >= data.minimum_deposit && amount >= min_amount
            && data.remaining_sale_amount >= amount, PreSaleError::InsufficientDeposit);

    let current_timestamp = ctx.accounts.clock.unix_timestamp as u64;

    require!(current_timestamp >= data.start_timestamp
            && current_timestamp <= data.end_timestamp,
            PreSaleError::TimeError);

    let pay_lamports = amount;

    let ix = transfer(&ctx.accounts.user.key(), &data.key(), pay_lamports);
    invoke(&ix, &[ctx.accounts.user.to_account_info(), data.to_account_info()])?;

    data.remaining_sale_amount -= amount;
    user_data.deposit += amount;

    msg!("user {} deposit {}", user_data.owner.key(), pay_lamports);

    Ok(())
}