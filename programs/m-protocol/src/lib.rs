use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;
pub mod constants;

// pub use state::*;
// use error::*;
pub use instructions::*;

declare_id!("8ifV5e1VBx8fYEA6tBBfBFYNme8u7AtwqvkrVxjqTPCR"); // devnet

// declare_id!("Fs9ajGLFFWcFqNWQx7wGzwFjTfyBCzwbu6MXqi3vSrro"); // localnet

#[program]
pub mod m_protocol {

    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>, name: String, limit: u64, vault_bump: u8, vault_pda_bump: u8) -> Result<()> {
        instructions::create_vault::handler(ctx, name, limit, vault_bump, vault_pda_bump)
    }

    pub fn create_user_vault_account(ctx: Context<CreateUserVaultAccount>, limit: u64, user_account_bump: u8) -> Result<()> {
        instructions::create_user_vault_account::handler(ctx, limit, user_account_bump)
    }

    pub fn deposit_to_user_vault_account(
        ctx: Context<DepositToUserVaultAccount>,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_to_user_vault_account::handler(ctx, amount)
    }

    pub fn withdraw_from_user_vault_account(
        ctx: Context<WithdrawFromUserVaultAccount>,
        amount: u64,
    ) -> Result<()> {
        instructions::withdraw_from_user_vault_account::handler(ctx, amount)
    }
    
    pub fn request_to_stake(
        ctx: Context<RequestToStake>,
        amount: u64,
        max: bool,
    ) -> Result<()> {
        instructions::request_to_stake::handler(ctx, amount, max)
    }
    
    pub fn request_to_unstake(
        ctx: Context<RequestToUnstake>,
        amount: u64,
        max: bool,
    ) -> Result<()> {
        instructions::request_to_unstake::handler(ctx, amount, max)
    }
    
    pub fn process_stake(
        ctx: Context<ProcessStake>,
    ) -> Result<()> {
        instructions::process_stake::handler(ctx)
    }
    
    pub fn process_unstake(
        ctx: Context<ProcessUnstake>,
    ) -> Result<()> {
        instructions::process_unstake::handler(ctx)
    }
    
    pub fn update_user_balance (
        ctx: Context<UpdateUserBalance>,
    ) -> Result<()> {
        instructions::update_user_balance::handler(ctx)
    }

    pub fn update_vault_balance (
        ctx: Context<UpdateVaultBalance>,
        // new_balance: u64 // todo: will update with vault token balance
    ) -> Result<()> {
        instructions::update_vault_balance::handler(ctx)
    }
    
    // ! not needed. Vault will not have limit
    pub fn update_vault_limit (
        ctx: Context<UpdateVaultLimit>,
        new_limit: u64
    ) -> Result<()> {
        instructions::update_vault_limit::handler(ctx, new_limit)
    }

    pub fn update_stake_request (
        ctx: Context<UpdateStakeRequest>,
        amount: u64,
        max: bool,
        cancel: bool
    ) -> Result<()> {
        instructions::update_stake_request::handler(ctx, amount, max, cancel)
    }
    
    pub fn update_unstake_request (
        ctx: Context<UpdateUnstakeRequest>,
        amount: u64,
        max: bool,
        cancel: bool
    ) -> Result<()> {
        instructions::update_unstake_request::handler(ctx, amount, max, cancel)
    }
    
    pub fn clear_stake_request (
        ctx: Context<ClearStakeRequest>,
    ) -> Result<()> {
        instructions::clear_stake_request::handler(ctx)
    }
    
    pub fn clear_unstake_request (
        ctx: Context<ClearUnstakeRequest>,
    ) -> Result<()> {
        instructions::clear_unstake_request::handler(ctx)
    }
    
    pub fn create_mango_account (
        ctx: Context<CreateAccountOnMango>,
        account_num: u64,
        mango_account_bump: u8,
    ) -> Result<()> {
        instructions::create_mango_account::handler(ctx, account_num, mango_account_bump)
    }
    
    pub fn close_mango_account (
        ctx: Context<CloseAccountOnMango>,
    ) -> Result<()> {
        instructions::close_mango_account::handler(ctx)
    }
    
    pub fn deposit_to_mango (
        ctx: Context<DepositToMango>,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_to_mango::handler(ctx, amount)
    }
    
    pub fn withdraw_from_mango (
        ctx: Context<WithdrawFromMango>,
        amount: u64,
    ) -> Result<()> {
        instructions::withdraw_from_mango::handler(ctx, amount)
    }
    
    pub fn delegate_mango_account (
        ctx: Context<DelegateMangoAccount>,
    ) -> Result<()> {
        instructions::delegate_mango_account::handler(ctx)
    }
}
