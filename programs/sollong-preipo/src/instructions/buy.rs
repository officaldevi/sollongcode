use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke};
use anchor_lang::solana_program::system_instruction::{transfer};
use merkletreers::node::{Node, Side};
use merkletreers::merkle_proof_check::merkle_proof_check;
use merkletreers::utils::hash_it;
use crate::errors::SollongError;
use crate::state::{RoundStock, UserData, Financial};

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = anchor_lang::solana_program::sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,
    #[account(mut, seeds = [& [round_stock.index][..], b"round-stock"], bump = round_stock.bump)]
    pub round_stock: Account<'info, RoundStock>,
    #[account(mut, seeds = [ & [round_stock.index][..], b"user-data", user.key().as_ref(),], bump = user_data.bump)]
    pub user_data: Account<'info, UserData>,
    
    #[account(mut, seeds = [ & [round_stock.index][..], b"financial", & [user.key().as_ref().last().unwrap() % round_stock.financial_index][..]], bump = financial.bump)]
    pub financial: Account<'info, Financial>,
    pub system_program: Program<'info, System>,
}

pub fn buy(ctx: Context<Buy>, buy_shares: u32) -> Result<()> {
    let round_stock = &mut ctx.accounts.round_stock;
    let user_data = &mut ctx.accounts.user_data;
    let financial = &mut ctx.accounts.financial;

    require!(!round_stock.whitelist_enabled, SollongError::FunctionCallError);

    let user_buy_shares = buy_shares + user_data.buy_shares;

    require!(buy_shares > 0
            && user_buy_shares >= round_stock.min_share
            && user_buy_shares <= round_stock.max_share
            && buy_shares <= round_stock.remaining_shares,
            SollongError::InsufficientShares);

    let current_timestamp = ctx.accounts.clock.unix_timestamp as u64;

    require!(current_timestamp >= round_stock.start_timestamp
            && current_timestamp <= round_stock.end_timestamp,
            SollongError::TimeError);

    let pay_lamports = buy_shares as u64 * round_stock.per_price;

    let ix = transfer(&ctx.accounts.user.key(), &financial.key(), pay_lamports);
    invoke(&ix, &[ctx.accounts.user.to_account_info(), financial.to_account_info()])?;

    round_stock.remaining_shares -= buy_shares;
    user_data.buy_shares += buy_shares;
    financial.total += pay_lamports;

    msg!("user {} buy {} shares and pay {} lamports to {}", user_data.owner.key(), buy_shares, pay_lamports, financial.key());

    Ok(())
}

pub fn buy_from_whitelist(ctx: Context<Buy>, buy_shares: u32, proofs: Vec<ProofNode>) -> Result<()> {
    let round_stock = &mut ctx.accounts.round_stock;
    let user_data = &mut ctx.accounts.user_data;
    let financial = &mut ctx.accounts.financial;

    let user_pub_key = ctx.accounts.user.key();

    
    require!(round_stock.whitelist_enabled, SollongError::FunctionCallError);

    let l: Vec<Node> = proofs.into_iter().map(ProofNode::into).collect();

    let mut leaf = [0u8; 32];
    hash_it(user_pub_key.as_ref(), &mut leaf);

    
    require!(round_stock.merkel_root_hash == merkle_proof_check(l, leaf), SollongError::UserNotVerified);

    let user_buy_shares = buy_shares + user_data.buy_shares;

    require!(buy_shares > 0
            && user_buy_shares >= round_stock.min_share
            && user_buy_shares <= round_stock.max_share
            && buy_shares <= round_stock.remaining_shares,
            SollongError::InsufficientShares);

    let current_timestamp = ctx.accounts.clock.unix_timestamp as u64;

    require!(current_timestamp >= round_stock.start_timestamp
            && current_timestamp <= round_stock.end_timestamp,
            SollongError::TimeError);


    let pay_lamports = buy_shares as u64 * round_stock.per_price;

    let ix = transfer(&ctx.accounts.user.key(), &financial.key(), pay_lamports);
    invoke(&ix, &[ctx.accounts.user.to_account_info(), financial.to_account_info()])?;

    round_stock.remaining_shares -= buy_shares;
    user_data.buy_shares += buy_shares;
    financial.total += pay_lamports;

    msg!("user {} buy {} shares and pay {} lamports to {}", user_data.owner.key(), buy_shares, pay_lamports, financial.key());

    Ok(())
}

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
            },
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
            },
        }
    }
}