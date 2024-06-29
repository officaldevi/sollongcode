use anchor_lang::prelude::*;
use crate::state::PreSaleData;

#[derive(Accounts)]
pub struct OwOperator<'info> {
    #[account()]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"presale-data"], bump = data.bump)]
    pub data: Account<'info, PreSaleData>,
}