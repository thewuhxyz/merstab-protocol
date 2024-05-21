use crate::state::*;
use crate::error::ErrorCode;
use crate::constants::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawFromUserVaultAccount<'info> {
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
    )]
    pub user_vault_account: Account<'info, UserVaultAccount>,

    #[account(
        mut, 
        // constraint = user_vault_usdc_token_account.mint == user_ata.mint,
        token::mint=user_ata.mint,
        token::authority=vault_pda_authority,
        address=user_vault_account.token_account @ ErrorCode::TokenAccountNotMatch
    )]
    pub user_vault_usdc_token_account: Account<'info, TokenAccount>,

    /// CHECK: vault pda authority
    #[account(
        seeds=[
            vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref()
        ],
        bump=vault.vault_authority_bump
    )]
    pub vault_pda_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        associated_token::mint=vault.mint,
        associated_token::authority=authority
    )]
    pub user_ata: Account<'info, TokenAccount>, // todo: change to regular token account

    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawFromUserVaultAccount<'info> {
    fn into_withdraw_from_user_vault_account_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_vault_usdc_token_account.to_account_info().clone(),
            to: self.user_ata.to_account_info().clone(),
            authority: self.vault_pda_authority.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handler (
    ctx: Context<WithdrawFromUserVaultAccount>,
    amount: u64,
    // max: bool,
) -> Result<()> {
    // let seed_signature = &[&[
    //             &ctx.accounts.vault.key().as_ref().to_owned(),
    //             VAULT_PDA_AUTHORITY_SEED.as_ref(),
    //             &[ctx.accounts.vault.vault_authority_bump],
    //         ]];

    token::transfer(
        ctx.accounts
            .into_withdraw_from_user_vault_account_context()
            .with_signer(&[&[
                &ctx.accounts.vault.key().as_ref().to_owned(),
                VAULT_PDA_AUTHORITY_SEED.as_ref(),
                &[ctx.accounts.vault.vault_authority_bump],
            ]]),
        amount,
    )?;

    ctx.accounts.user_vault_account.withdrawal = ctx.accounts.user_vault_account.withdrawal.checked_add(amount).unwrap();
    Ok(())
}