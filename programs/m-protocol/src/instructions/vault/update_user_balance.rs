use crate::state::*;
use crate::error::ErrorCode;

use anchor_lang::prelude::*;
// use rust_decimal::prelude::ToPrimitive;

#[derive(Accounts)]
pub struct UpdateUserBalance<'info> {
    #[account(
        has_one=manager @ ErrorCode::NotAdmin
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut, 
        has_one=vault @ ErrorCode::VaultNotMatch,
        seeds=[
            user_vault_account.vault.key().as_ref(),
            user_vault_account.authority.key().as_ref(),
        ],
        bump=user_vault_account.user_account_bump,
    )]
    pub user_vault_account: Account<'info, UserVaultAccount>,

    pub manager: Signer<'info>,
}

impl<'info> UpdateUserBalance<'info> {
    fn calculate_user_equity(&self) -> u64 {
        self.user_vault_account.equity
        .checked_mul(self.vault.total_equity).unwrap() 
        .checked_div(self.vault.previous_total_equity).unwrap()
    }
}

pub fn handler(ctx: Context<UpdateUserBalance>) -> Result<()> {
    ctx.accounts.user_vault_account.equity = ctx.accounts.calculate_user_equity();
    ctx.accounts.user_vault_account.user_pnl = ctx.accounts.user_vault_account.calculate_pnl();
    Ok(())
}
