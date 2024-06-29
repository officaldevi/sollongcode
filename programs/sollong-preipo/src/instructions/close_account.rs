use anchor_lang::prelude::*;
use crate::errors::SollongError;
use crate::state::RoundStock;

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    pub user: Signer<'info>,
    #[account(mut, close = to)]
    pub data: Account<'info, RoundStock>,
    /// CHECK: withdraw to address
    #[account(mut)]
    pub to: AccountInfo<'info>,
}
pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
    // require!(ctx.accounts.user.key() == ctx.accounts.data.ow, SollongError::OwError);
    // ctx.accounts.data.close(ctx.accounts.to.to_account_info())?;
    Ok(())
}