use crate::constants::MAX_REQUESTS;
use crate::state::*;
use crate::error::ErrorCode;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClearUnstakeRequest<'info> {
    #[account(
        has_one=manager @ ErrorCode::NotAdmin, 
        has_one=unstake_request_account @ ErrorCode::VaultNotMatch
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut, 
        has_one=vault @ ErrorCode::VaultNotMatch,
    )]
    pub unstake_request_account: AccountLoader<'info, UnstakeReq>,

    pub manager: Signer<'info>,
}

pub fn handler (ctx: Context<ClearUnstakeRequest>) -> Result<()> {
    let mut unstake_req = ctx.accounts.unstake_request_account.load_mut()?;
    unstake_req.count = 0;
    unstake_req.orders = [Pubkey::default(); MAX_REQUESTS];
    
    Ok(())
}