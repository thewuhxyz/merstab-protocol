use crate::state::*;
use crate::constants::*;
use crate::error::ErrorCode;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DepositToUserVaultAccount<'info> {
    
    #[account(
        mut, 
        constraint= vault.limit >= amount @ ErrorCode::MaxVaultLimit
    )]
    pub vault: Account<'info, Vault>, 
    
    #[account(
        mut,
        seeds=[
            user_vault_account.vault.key().as_ref(),
            user_vault_account.authority.key().as_ref(),
        ],
        bump=user_vault_account.user_account_bump,
        has_one=authority @ ErrorCode::WrongUserAccountAuthority, 
        has_one=vault @ ErrorCode::VaultNotMatch, 
        constraint=user_vault_account.deposit_limit>=user_vault_account.deposit @ ErrorCode::MaxDepositLimit
    )]
    pub user_vault_account: Account<'info, UserVaultAccount>,
    
    #[account(
        mut, 
        token::mint=user_ata.mint,
        token::authority=vault_pda_authority,
        address=user_vault_account.token_account @ ErrorCode::TokenAccountNotMatch,
        constraint=user_vault_account.deposit_limit>=amount @ ErrorCode::MaxDepositLimit,        
    )]
    pub user_vault_usdc_token_account: Account<'info, TokenAccount>,

    /// CHECK: user authority
    #[account(
        mut,
        address=user_vault_account.authority @ ErrorCode::WrongUserAccountAuthority
    )]
    pub authority: Signer<'info>,
    
    /// CHECK: user authority
    #[account(
        seeds=[
            vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref()
        ],
        bump=vault.vault_authority_bump
    )]
    pub vault_pda_authority: UncheckedAccount<'info>,


    #[account(
        mut,
        associated_token::mint=vault.mint,
        associated_token::authority=authority
    )]
    pub user_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl <'info> DepositToUserVaultAccount <'info> {
    fn into_deposit_to_user_vault_account_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_ata.to_account_info().clone(),
            to: self.user_vault_usdc_token_account.to_account_info().clone(),
            authority: self.authority.to_account_info().clone()
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
        
    }
}

pub fn handler (
    ctx: Context<DepositToUserVaultAccount>,
    amount: u64,
) -> Result<()> {
    token::transfer(ctx.accounts.into_deposit_to_user_vault_account_context(), amount)?;

    ctx.accounts.user_vault_account.deposit = ctx.accounts.user_vault_account.deposit.checked_add(amount).unwrap();
    ctx.accounts.user_vault_account.deposit_limit = ctx.accounts.user_vault_account.deposit_limit.checked_sub(amount).unwrap();
    
    ctx.accounts.vault.deposit = ctx.accounts.vault.deposit.checked_add(amount).unwrap();
    ctx.accounts.vault.limit = ctx.accounts.vault.limit.checked_sub(amount).unwrap();

    Ok(())
}