mod state;
mod instructions;
mod errors;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("");

#[program]
pub mod sollong_preipo {
    use super::*;

    
    pub fn init(ctx: Context<Init>) -> Result<()> {
        instructions::init(ctx)
    }

    
    pub fn new_round(ctx: Context<NewRound>, total_share: u32, min_share: u32, max_share:u32, per_price: u64) -> Result<()> {
        instructions::new_round(ctx, total_share, min_share, max_share, per_price)
    }

    
    pub fn create_financial_account(ctx: Context<CreateFinancial>) -> Result<()> {
        instructions::create_financial_account(ctx)
    }

    
    pub fn create_user_account(ctx: Context<CreateUserData>) ->Result<()> {
        instructions::create_user_account(ctx)
    }

    
    pub fn set_merkel_tree_hash(ctx: Context<OwOpInRound>, new_list: [u8; 32]) -> Result<()> {
        instructions::set_merkel_tree_hash(ctx, new_list)
    }

    
    pub fn set_white_list_status(ctx: Context<OwOpInRound>, is_enabled: bool) -> Result<()> {
        instructions::set_white_list_status(ctx, is_enabled)
    }

    
    pub fn buy(ctx: Context<Buy>, buy_shares: u32) -> Result<()> {
        instructions::buy(ctx, buy_shares)
    }

    
    pub fn buy_from_whitelist(ctx: Context<Buy>, buy_shares: u32, proofs: Vec<ProofNode>) -> Result<()> {
        instructions::buy_from_whitelist(ctx, buy_shares, proofs)
    }

    
    pub fn change_owner(ctx: Context<OwOp>, new_owner: Pubkey) -> Result<()>{
        instructions::change_owner(ctx, new_owner)
    }

    
    pub fn set_time(ctx: Context<OwOpInRound>, start_time: u64, end_time: u64) -> Result<()> {
        instructions::set_time(ctx, start_time, end_time)
    }

    
    pub fn set_buy_share_limit(ctx: Context<OwOpInRound>, min_share: u32, max_share: u32) -> Result<()> {
        instructions::set_buy_share_limit(ctx, min_share, max_share)
    }

    
    pub fn owner_withdraw(ctx: Context<OwWithdraw>) -> Result<()> {
        instructions::owner_withdraw(ctx)
    }

    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        instructions::close_account(ctx)
    }
}

