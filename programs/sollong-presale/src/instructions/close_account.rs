use anchor_lang::prelude::*;
use crate::errors::PreSaleError;
use crate::state::PreSaleData;

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    pub user: Signer<'info>,
    #[account(mut, close = to)]
    pub data: Account<'info, PreSaleData>,
    /// CHECK: withdraw to address
    #[account(mut)]
    pub to: AccountInfo<'info>,
}

pub fn close_account(ctx: Context<CloseAccount>) -> anchor_lang::Result<()> {
    require!(ctx.accounts.user.key() == ctx.accounts.data.owner, PreSaleError::OwError);
    // ctx.accounts.data.close(ctx.accounts.to.to_account_info())?;
    Ok(())
}