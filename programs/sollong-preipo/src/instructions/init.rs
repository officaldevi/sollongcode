use anchor_lang::prelude::*;
use crate::state::Metadata;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::bpf_loader_upgradeable;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub current_authority: Signer<'info>,
    #[account(init, payer = current_authority, space=42 + 8 + 32, seeds = [b"metadata"], bump)]
    pub metadata: Account<'info, Metadata>,
    #[account(address = anchor_lang::solana_program::sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,
    #[account(mut, seeds = [crate::ID.as_ref()], seeds::program = bpf_loader_upgradeable::ID, bump)]
    pub program_data: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the BPF upgradeable loader program
    #[account(address = bpf_loader_upgradeable::ID)]
    pub bpf_upgradeable_loader: AccountInfo<'info>,
}

pub fn init(ctx: Context<Init>, withdraw_to: Pubkey) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    metadata.bump = ctx.bumps.metadata;
    metadata.owner = ctx.accounts.current_authority.key();
    // It is expected to use a multi-signature address to receive assets
    metadata.withdraw_to = withdraw_to;
    // owner will be valid for one day
    metadata.owner_validate_time = (ctx.accounts.clock.unix_timestamp as u64) + 86400;

    msg!("Sollong Program Init ...");

    // Set the upgrade authority to the "11111111111111111111111111111111" address
    // Discard program upgrade permissions
    let new_authority = Pubkey::default();
    let current_authority_info = &ctx.accounts.current_authority;
    let program_id = ctx.program_id;

    let ix = bpf_loader_upgradeable::set_upgrade_authority(
        program_id,
        &current_authority_info.key,
        Some(&new_authority),
    );

    invoke(
        &ix,
        &[
            current_authority_info.to_account_info(),
            ctx.accounts.program_data.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.bpf_upgradeable_loader.to_account_info(),
        ],
    )?;

    Ok(())
}
