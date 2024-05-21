use crate::state::*;
use crate::error::ErrorCode;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateVaultLimit<'info> {
    #[account(
        mut, 
        has_one=manager @ ErrorCode::NotAdmin,
    )]
    pub vault: Account<'info, Vault>,

    pub manager: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateVaultLimit>, new_limit: u64) -> Result<()> {
    ctx.accounts.vault.limit = new_limit;
    Ok(())
}
