use crate::state::*;
use crate::error::ErrorCode;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RequestToUnstake <'info> {
    #[account(
        mut,
        seeds=[
            user_vault_account.vault.key().as_ref(),
            user_vault_account.authority.key().as_ref(),
        ],
        bump=user_vault_account.user_account_bump,
        has_one=authority @ ErrorCode::WrongUserAccountAuthority,
        constraint=user_vault_account.user_unstake.unstake_request_active==false @ ErrorCode::UnstakeRequestActive,
        
    )]
    pub user_vault_account: Account<'info, UserVaultAccount>,
    
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        constraint=vault_unstake_req_account.load()?.vault==user_vault_account.vault @ ErrorCode::VaultNotMatch,
        constraint=vault_unstake_req_account.load()?.max_requests>=vault_unstake_req_account.load()?.count @ ErrorCode::MaxRequestLimit
    )]
    pub vault_unstake_req_account: AccountLoader<'info, UnstakeReq>
}

pub fn handler ( ctx: Context<RequestToUnstake>, amount: u64, max: bool) -> Result<()> {

    let mut unstake_req_account = ctx.accounts.vault_unstake_req_account.load_mut()?;
    let index = unstake_req_account.count;

    ctx.accounts.user_vault_account.user_unstake.unstake_request_active = true;
    ctx.accounts.user_vault_account.user_unstake.unstake_amount = amount;
    ctx.accounts.user_vault_account.user_unstake.max = max;
    ctx.accounts.user_vault_account.user_unstake.cancel = false;
    ctx.accounts.user_vault_account.user_unstake.status = RequestStatus::Pending;

    unstake_req_account.orders[index as usize] = ctx.accounts.user_vault_account.key();
    unstake_req_account.count = index + 1;

    Ok(())
} 