use anchor_lang::prelude::*;
use crate::state::PreSaleData;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space=81, seeds = [b"presale-data"], bump)]
    pub data: Account<'info, PreSaleData>,
    #[account(address = anchor_lang::solana_program::sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>
}

pub fn initialize(ctx: Context<Init>) -> anchor_lang::Result<()> {
    let total_deposit = 500_0000u64 * 10u64.pow(9);
    let minimum_deposit = 2u64 * 10u64.pow(9);

    let data = &mut ctx.accounts.data;
    data.owner = ctx.accounts.user.key();
    data.total_sale_amount = total_deposit;
    data.remaining_sale_amount = total_deposit;
    data.minimum_deposit = minimum_deposit;
    data.start_timestamp = ctx.accounts.clock.unix_timestamp as u64;
    data.end_timestamp = data.start_timestamp  + 100 * 86400;
    data.bump = ctx.bumps.data;

    msg!("Sollong Pre-Sale Program data Initialized");
    Ok(())
}