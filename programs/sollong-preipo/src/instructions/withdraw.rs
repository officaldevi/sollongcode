use anchor_lang::prelude::*;
use crate::errors::SollongError;
use crate::state::{Financial, Metadata};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account()]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"metadata"], bump = metadata.bump)]
    pub metadata: Account<'info, Metadata>,
    #[account(mut, seeds = [ &[financial.round_index][..], b"financial", &[financial.index][..]], bump = financial.bump)]
    pub financial: Account<'info, Financial>,
    /// CHECK: The address has been designated as a specific address
    #[account(mut, address = metadata.withdraw_to)]
    pub to: AccountInfo<'info>,
}

// Since its withdrawal address is a specially designated multi-signature wallet address,
// anyone withdrawing money here will not trigger security risks,
// and since the OW permission is given up at the beginning,
// no user will be prohibited from triggering this method, which is convenient for later actions.
pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    let financial = &mut ctx.accounts.financial;
    let to = &mut ctx.accounts.to;

    let rent_balance = Rent::get()?.minimum_balance(financial.to_account_info().data_len());

    let amount = **financial.to_account_info().lamports.borrow() - rent_balance;

    require!(amount > 0, SollongError::InsufficientBalance);

    **financial.to_account_info().try_borrow_mut_lamports()? -= amount;
    **to.to_account_info().try_borrow_mut_lamports()? += amount;

    Ok(())
}
