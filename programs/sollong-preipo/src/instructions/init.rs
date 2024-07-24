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
    /// CHECK: The withdrawal address is manually specified by the owner and does not require detection
    #[account(mut)]
    pub withdraw_to: AccountInfo<'info>,
    #[account(mut, address=crate::ID)]
    pub program: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

pub fn init(ctx: Context<Init>) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    metadata.bump = ctx.bumps.metadata;
    metadata.owner = ctx.accounts.current_authority.key();
    // It is expected to use a multi-signature address to receive assets
    metadata.withdraw_to = ctx.accounts.withdraw_to.key();
    // owner will be valid for one day
    metadata.owner_validate_time = (ctx.accounts.clock.unix_timestamp as u64) + 86400;

    msg!("Sollong Program Init ...");

    // Set the upgrade authority to the "11111111111111111111111111111111" address
    // Discard program upgrade permissions
    let new_authority = Pubkey::default();
    let current_authority_info = &ctx.accounts.current_authority;
    let program_info = &ctx.accounts.program;

    let ix = bpf_loader_upgradeable::set_upgrade_authority(
        &program_info.key,
        &current_authority_info.key,
        Some(&new_authority),
    );

    invoke(
        &ix,
        &[
            program_info.clone(),
            current_authority_info.to_account_info(),
        ],
    )?;

    Ok(())
}
