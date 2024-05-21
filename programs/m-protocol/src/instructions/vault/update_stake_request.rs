use crate::state::*;
use crate::error::ErrorCode;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateStakeRequest <'info> {

    pub vault: Account<'info, Vault>,

    #[account(
        mut, 
        seeds=[
            user_vault_account.vault.key().as_ref(),
            user_vault_account.authority.key().as_ref(),
        ],
        bump=user_vault_account.user_account_bump,
        has_one=authority @ ErrorCode::WrongUserAccountAuthority, 
        has_one=vault @ ErrorCode::VaultNotMatch
    )]
    pub user_vault_account: Account<'info, UserVaultAccount>,

    pub authority: Signer<'info>,

}

pub fn handler ( ctx: Context<UpdateStakeRequest>, amount: u64, max: bool, cancel: bool) -> Result<()> {

    ctx.accounts.user_vault_account.user_stake.stake_amount = amount;
    ctx.accounts.user_vault_account.user_stake.cancel = cancel;
    ctx.accounts.user_vault_account.user_stake.max = max;
    

    Ok(())
} 