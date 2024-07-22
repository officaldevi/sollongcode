use anchor_lang::prelude::*;
use crate::errors::SollongError;
use crate::state::{Financial, Metadata, RoundStock};

#[derive(Accounts)]
pub struct OwWithdraw<'info> {
    #[account()]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"metadata"], bump = metadata.bump)]
    pub metadata: Account<'info, Metadata>,
    #[account(mut, seeds = [ &[financial.round_index][..], b"financial", &[financial.index][..]], bump = financial.bump)]
    pub financial: Account<'info, Financial>,
    /// CHECK: The withdrawal address is manually specified by the owner and does not require detection
    #[account(mut)]
    pub to: AccountInfo<'info>,
}
pub fn owner_withdraw(ctx: Context<OwWithdraw>) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let financial = &mut ctx.accounts.financial;
    let to = &mut ctx.accounts.to;

    require!(ctx.accounts.user.key() == metadata.owner , SollongError::OwError);

    let rent_balance = Rent::get()?.minimum_balance(financial.to_account_info().data_len());

    let amount = **financial.to_account_info().lamports.borrow() - rent_balance;

    require!(amount > 0, SollongError::InsufficientBalance);

    **financial.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;

    Ok(())
}
