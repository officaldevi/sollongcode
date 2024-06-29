mod state;
mod instructions;
mod errors;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("");

#[program]
pub mod sollong_bank {
    use super::*;

    
    pub fn init(ctx: Context<Init>) -> Result<()> {
        instructions::init(ctx)
    }

    
    pub fn create_financial_account(ctx: Context<CreateFinancial>) -> Result<()> {
        instructions::create_financial_account(ctx)
    }

    
    pub fn create_user_account(ctx: Context<CreateUserData>) ->Result<()> {
        instructions::create_user_account(ctx)
    }

    
    pub fn deposit(ctx: Context<Deposit>, deposit_amount: u64) -> Result<()> {
        instructions::deposit(ctx, deposit_amount)
    }

    
    pub fn withdraw() -> Result<()> {
        Ok(())
    }

    
    pub fn change_owner(ctx: Context<OwOp>, new_owner: Pubkey) -> Result<()>{
        instructions::change_owner(ctx, new_owner)
    }

    
    pub fn owner_withdraw(ctx: Context<OwWithdraw>) -> Result<()> {
        instructions::owner_withdraw(ctx)
    }

    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        instructions::close_account(ctx)
    }
}