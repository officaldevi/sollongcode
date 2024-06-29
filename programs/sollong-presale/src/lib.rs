pub mod errors;
pub mod state;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("");

#[program]
pub mod sollong_presale {
    use super::*;
    pub fn initialize(ctx: Context<Init>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn create_user_account(ctx: Context<InitUserData>) ->Result<()> {
       instructions::init_user_data(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) ->Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn change_owner(ctx: Context<OwOperator>, new_owner: Pubkey) -> Result<()>{
        instructions::change_owner(ctx, new_owner)
    }

    pub fn set_time(ctx: Context<OwOperator>, start_time: u64, end_time: u64) -> Result<()> {
        instructions::set_time(ctx, start_time, end_time)
    }

    pub fn set_deposit_limit(ctx: Context<OwOperator>, minimum_deposit: u64) -> Result<()> {
        instructions::set_deposit_limit(ctx, minimum_deposit)
    }

    pub fn set_total_sale(ctx: Context<OwOperator>, total_sale: u64) -> Result<()> {
        instructions::set_total_sale(ctx, total_sale)
    }

    pub fn owner_withdraw(ctx: Context<OwWithdraw>) -> Result<()> {
        instructions::owner_withdraw(ctx)
    }

    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        instructions::close_account(ctx)
    }
}
