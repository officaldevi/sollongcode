use anchor_lang::prelude::*;
use anchor_lang::prelude::borsh::{BorshDeserialize};
use anchor_lang::solana_program::program::{invoke};
use anchor_lang::solana_program::system_instruction::{transfer};
use anchor_lang::solana_program::sysvar::clock::Clock;
use merkletreers::node::{Node, Side};
use merkletreers::merkle_proof_check::merkle_proof_check;
use merkletreers::utils::hash_it;

declare_id!("");

#[program]
pub mod sollong_ido {
    use super::*;

    // init functional
    pub fn initialize(ctx: Context<InitIdo>) -> Result<()> {
        let total_shares = 200u32;
        let min_share = 1u32;
        let max_share = 10u32;
        let per_price: u64 = 15 * 10u64.pow(8);

        let ido_stock = &mut ctx.accounts.ido_stock;
        ido_stock.ow = ctx.accounts.user.key();
        ido_stock.total_shares = total_shares;
        ido_stock.remaining_shares = ido_stock.total_shares;
        ido_stock.min_share = min_share;
        ido_stock.max_share = max_share;
        ido_stock.per_price = per_price;
        ido_stock.bump = ctx.bumps.ido_stock;

        msg!("Sollong IDO Program data Initialized");
        Ok(())
    }

    pub fn create_financial_accounts(ctx: Context<CreateFinancial>) -> Result<()> {
        require!(ctx.accounts.user.key() == ctx.accounts.ido_stock.ow, IdoError::OwError);

        let financial = &mut ctx.accounts.financial;
        financial.bump = ctx.bumps.financial;
        financial.total = 0;

        let data = &mut ctx.accounts.ido_stock;

        msg!("Create New Financial {} -> {}", data.financial_index, financial.key());
        data.financial_index += 1;

        Ok(())
    }

    pub fn create_user_account(ctx: Context<CreateUserData>) ->Result<()> {
        let user_data = &mut ctx.accounts.data;
        user_data.ow = ctx.accounts.user.key();
        user_data.bump = ctx.bumps.data;

        msg!("User data create: {} -> {}",  user_data.ow.key(), user_data.key());
        Ok(())
    }

    
    
    
    pub fn buy(ctx: Context<Buy>, buy_shares: u32)->Result<()> {
        let ido_stock = &mut ctx.accounts.ido_stock;
        let user_data = &mut ctx.accounts.data;
        let financial = &mut ctx.accounts.financial;

        require!(!ido_stock.whitelist_status, IdoError::FunctionCallError);

        let user_buy_shares = buy_shares + user_data.buy_shares;

        require!(buy_shares > 0
            && buy_shares >= ido_stock.min_share
            && user_buy_shares <= ido_stock.max_share
            && buy_shares <= ido_stock.remaining_shares,
            IdoError::InsufficientShares);

        let current_timestamp = ctx.accounts.clock.unix_timestamp as u64;

        require!(current_timestamp >= ido_stock.start_timestamp
            && current_timestamp <= ido_stock.end_timestamp,
            IdoError::TimeError);

        let pay_lamports = buy_shares as u64 * ido_stock.per_price;

        let ix = transfer(&ctx.accounts.user.key(), &financial.key(), pay_lamports);
        invoke(&ix, &[ctx.accounts.user.to_account_info(), financial.to_account_info()])?;

        ido_stock.remaining_shares -= buy_shares;
        user_data.buy_shares += buy_shares;
        financial.total += pay_lamports;

        emit!(BuyEvent{
            buyer: user_data.ow.key(),
            shares: buy_shares,
            pay_lamports
        });

        msg!("user {} buy {} shares and pay {} lamports to {}", user_data.ow.key(), buy_shares, pay_lamports, financial.key());

        Ok(())
    }

    pub fn buy_from_whitelist(ctx: Context<Buy>, buy_shares: u32, proofs: Vec<ProofNode>)->Result<()> {
        let ido_stock = &mut ctx.accounts.ido_stock;
        let user_data = &mut ctx.accounts.data;
        let financial = &mut ctx.accounts.financial;

        let user_pub_key = ctx.accounts.user.key();

        
        require!(ido_stock.whitelist_status, IdoError::FunctionCallError);

        let l: Vec<Node> = proofs.into_iter().map(ProofNode::into).collect();

        let mut leaf = [0u8; 32];
        hash_it(user_pub_key.as_ref(), &mut leaf);

        
        require!(ido_stock.white_list == merkle_proof_check(l, leaf), IdoError::UserNotVerified);

        let user_buy_shares = buy_shares + user_data.buy_shares;

        
        require!(user_buy_shares >= ido_stock.min_share
            && user_buy_shares <= ido_stock.max_share
            && buy_shares <= ido_stock.remaining_shares, IdoError::InsufficientShares);

        let current_timestamp = ctx.accounts.clock.unix_timestamp as u64;

        require!(current_timestamp >= ido_stock.start_timestamp
            && current_timestamp <= ido_stock.end_timestamp
            ,IdoError::TimeError);

        let pay_lamports = buy_shares as u64 * ido_stock.per_price;

        let ix = transfer(&ctx.accounts.user.key(), &financial.key(), pay_lamports);
        invoke(&ix, &[ctx.accounts.user.to_account_info(), financial.to_account_info()])?;

        ido_stock.remaining_shares -= buy_shares;
        user_data.buy_shares += buy_shares;
        financial.total += pay_lamports;

        msg!("user {} buy {} shares and pay {} lamports to {}", user_data.ow.key(), buy_shares, pay_lamports, financial.key());

        Ok(())
    }

    pub fn change_owner(ctx: Context<ChangeOw>, new_owner: Pubkey) -> Result<()>{
        let ido_stock = &mut ctx.accounts.ido_stock;

        if new_owner == ctx.accounts.user.key() {
            return Ok(());
        }

        require!(ido_stock.ow == ctx.accounts.user.key(), IdoError::OwError);

        ido_stock.ow = new_owner;

        Ok(())
    }

    pub fn set_white_list(ctx: Context<SetWhiteList>, new_list: [u8; 32]) -> Result<()> {
        let ido_stock = &mut ctx.accounts.ido_stock;

        require!(ctx.accounts.user.key() == ido_stock.ow, IdoError::OwError);

        ido_stock.white_list = new_list;
        ido_stock.whitelist_status = true;

        Ok(())
    }

    pub fn set_time(ctx: Context<OwOp>, start_time: u64, end_time: u64) -> Result<()> {
        let ido_stock = &mut ctx.accounts.ido_stock;

        require!(ctx.accounts.user.key() == ido_stock.ow, IdoError::OwError);

        ido_stock.start_timestamp = start_time;
        ido_stock.end_timestamp = end_time;
        Ok(())
    }

    pub fn set_buy_share_limit(ctx: Context<OwOp>, min_share: u32, max_share: u32) -> Result<()> {
        let ido_stock = &mut ctx.accounts.ido_stock;

        require!(ctx.accounts.user.key() == ido_stock.ow, IdoError::OwError);

        ido_stock.min_share = min_share;
        ido_stock.max_share = max_share;

        Ok(())
    }

    pub fn disable_white_list(ctx: Context<OwOp>) -> Result<()> {
        let ido_stock = &mut ctx.accounts.ido_stock;

        require!(ctx.accounts.user.key() == ido_stock.ow, IdoError::OwError);

        ido_stock.whitelist_status = false;

        Ok(())
    }

    pub fn owner_withdraw(ctx: Context<OwWithdraw>) -> Result<()> {
        let ido_stock = &mut ctx.accounts.ido_stock;
        let financial = &mut ctx.accounts.financial;
        let to = &mut ctx.accounts.to;

        require!(ctx.accounts.user.key() == ido_stock.ow, IdoError::OwError);

        let rent_balance = Rent::get()?.minimum_balance(financial.to_account_info().data_len());

        let amount = **financial.to_account_info().lamports.borrow() - rent_balance;

        **financial.to_account_info().try_borrow_mut_lamports()? -= amount;
        **to.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        require!(ctx.accounts.user.key() == ctx.accounts.data.ow, IdoError::OwError);
        // ctx.accounts.data.close(ctx.accounts.to.to_account_info())?;
        Ok(())
    }
}

#[error_code]
pub enum IdoError {
    #[msg("The available shares for purchase are insufficient or exceed the limit")]
    InsufficientShares,
    #[msg("Not in the current participation time period.")]
    TimeError,
    #[msg("Only the owner can call")]
    OwError,
    #[msg("Merkle Tree Error")]
    MerkleError,
    #[msg("Please call the 'buy' method for non-whitelisted purchases")]
    FunctionCallError,
    #[msg("The user does not have purchasing qualifications")]
    UserNotVerified
}

#[account]
pub struct IdoStock {
    pub ow: Pubkey,
    pub financial_index: u8,
    pub whitelist_status: bool,
    pub white_list: [u8;32],
    pub total_shares: u32,
    pub remaining_shares: u32,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub min_share: u32,
    pub max_share: u32,
    pub per_price: u64,
    bump: u8
}

#[account]
pub struct Financial {
    pub total: u64,
    bump: u8
}

#[derive(Accounts)]
pub struct InitIdo<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space=256, seeds = [b"ido-stock"], bump)]
    pub ido_stock: Account<'info, IdoStock>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct UserData {
    ow: Pubkey,
    buy_shares: u32,
    bump: u8
}

#[derive(Accounts)]
pub struct CreateUserData<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space=64, seeds = [b"user-data", user.key().as_ref()], bump)]
    pub data: Account<'info, UserData>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = anchor_lang::solana_program::sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,
    #[account(mut, seeds = [b"ido-stock"], bump = ido_stock.bump)]
    pub ido_stock: Account<'info, IdoStock>,
    #[account(mut, seeds = [b"user-data", user.key().as_ref()], bump = data.bump)]
    pub data: Account<'info, UserData>,
    #[account(mut, seeds = [b"financial", &[user.key().as_ref().last().unwrap() % ido_stock.financial_index][..]], bump = financial.bump)]
    pub financial: Account<'info, Financial>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct OwOp<'info> {
    #[account()]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"ido-stock"], bump = ido_stock.bump)]
    pub ido_stock: Account<'info, IdoStock>,
}


#[derive(Accounts)]
pub struct ChangeOw<'info> {
    #[account()]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"ido-stock"], bump = ido_stock.bump)]
    pub ido_stock: Account<'info, IdoStock>,
}

#[derive(Accounts)]
pub struct SetWhiteList<'info> {
    #[account()]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"ido-stock"], bump = ido_stock.bump)]
    pub ido_stock: Account<'info, IdoStock>
}

#[derive(Accounts)]
pub struct CreateFinancial<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"ido-stock"], bump = ido_stock.bump)]
    pub ido_stock: Account<'info, IdoStock>,
    #[account(init, payer = user, space=64, seeds = ["financial".as_bytes(), &[ido_stock.financial_index][..]], bump)]
    pub financial: Account<'info, Financial>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct OwWithdraw<'info> {
    #[account()]
    pub user: Signer<'info>,
    #[account(mut)]
    pub financial: Account<'info, Financial>,
    /// CHECK: withdraw to address
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(mut, seeds = [b"ido-stock"], bump = ido_stock.bump)]
    pub ido_stock: Account<'info, IdoStock>,
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    pub user: Signer<'info>,
    #[account(mut, close = to)]
    pub data: Account<'info, IdoStock>,
    /// CHECK: withdraw to address
    #[account(mut)]
    pub to: AccountInfo<'info>,
}

#[event]
pub struct BuyEvent {
    buyer: Pubkey,
    shares: u32,
    pay_lamports: u64
}

// #[derive(Accounts)]
// pub struct BuyIdo<'info> {
//     #[account(mut, signer)]

//     pub authority: AccountInfo<'info>,

//     #[account(mut)]
//     pub user: Signer<'info>,

//     pub system_program: Program<'info, System>,
// }

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct ProofNode {
    pub data: [u8; 32],
    pub side: u8,
}

impl From<Node> for ProofNode {
    fn from(node: Node) -> Self {
        ProofNode {
            data: node.data,
            side: match node.side {
                Side::LEFT => 0u8,
                Side::RIGHT => 1u8,
            }
        }
    }
}

impl Into<Node> for ProofNode {
    fn into(self) -> Node {
        Node {
            data: self.data,
            side: match self.side {
                0u8 => Side::LEFT,
                1u8 => Side::RIGHT,
                _ => panic!("Invalid value for Side enum, must be either `0` or `1`"),
            }
        }
    }
}