use crate::constants::MAX_REQUESTS;
use crate::state::*;
use crate::error::ErrorCode;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClearStakeRequest<'info> {
    #[account(
        has_one=manager@ErrorCode::NotAdmin, 
        has_one=stake_request_account @ErrorCode::VaultNotMatch
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut, 
        has_one=vault@ErrorCode::VaultNotMatch,
    )]
    pub stake_request_account: AccountLoader<'info, StakeReq>,

    pub manager: Signer<'info>,
}

pub fn handler (ctx: Context<ClearStakeRequest>) -> Result<()> {
    let mut stake_req = ctx.accounts.stake_request_account.load_mut()?;
    stake_req.count = 0;
    stake_req.orders = [Pubkey::default(); MAX_REQUESTS];
    Ok(())
}