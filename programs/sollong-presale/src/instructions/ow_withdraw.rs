use anchor_lang::prelude::*;
use crate::errors::PreSaleError;
use crate::state::PreSaleData;

#[derive(Accounts)]
pub struct OwWithdraw<'info> {
    #[account()]
    pub user: Signer<'info>,
    /// CHECK: withdraw to address
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(mut, seeds = [b"presale-data"], bump = data.bump)]
    pub data: Account<'info, PreSaleData>,
}

pub fn owner_withdraw(ctx: Context<OwWithdraw>) -> anchor_lang::Result<()> {
    let data = &mut ctx.accounts.data;
    let to = &mut ctx.accounts.to;

    require!(ctx.accounts.user.key() == data.owner, PreSaleError::OwError);

    let rent_balance = Rent::get()?.minimum_balance(data.to_account_info().data_len());

    let amount = **data.to_account_info().lamports.borrow() - rent_balance;

    **data.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;

    msg!("Withdraw {} lamports to {}", amount, ctx.accounts.to.key());
    Ok(())
}