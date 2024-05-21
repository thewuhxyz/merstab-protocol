use crate::state::*;
use crate::constants::*;
use crate::error::ErrorCode;


use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

#[derive(Accounts)]
pub struct CreateUserVaultAccount<'info> {

    #[account(
        init,
        payer=user_account_authority,
        seeds=[
            vault.key().as_ref(),
            user_account_authority.key().as_ref(),
        ],
        bump,
        space=8+512 // todo: adjust space 
    )]
    pub user_vault_account: Box<Account<'info, UserVaultAccount>>,

    #[account(
        init,
        payer=user_account_authority,
        seeds=[
            vault.key().as_ref(), 
            user_account_authority.key().as_ref(),
            b"usdc".as_ref()
        ],
        bump,
        token::mint = token_mint,
        token::authority = vault_pda_authority, // * should be pda
        )]
    pub token_account: Account<'info, TokenAccount>,
        
    pub vault: Box<Account<'info, Vault>>,

    // #[account(mut)]
    // pub payer: Signer<'info>,

    /// CHECK: vault pda authority
    #[account(
        seeds=[
            vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref()
        ],
        bump=vault.vault_authority_bump
    )]
    pub vault_pda_authority: UncheckedAccount<'info>,

    /// CHECK: user authority 
    #[account(mut)]
    pub user_account_authority: Signer<'info>,
    
    #[account(
        mut, 
        address=vault.mint @ ErrorCode::WrongMintProvided
    )]
    pub token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler ( ctx: Context<CreateUserVaultAccount>, limit: u64, user_account_bump: u8) -> Result<()>{

    let user_account_key = Pubkey::create_program_address(
        &[
            ctx.accounts.vault.key().as_ref(), 
            ctx.accounts.user_account_authority.key().as_ref(), 
            &[user_account_bump]
            ], 
            ctx.program_id
        ).unwrap_or_default();
    
    if user_account_key != ctx.accounts.user_vault_account.key() {
        return err!(ErrorCode::BumpNotMatch)
    }

    // user_vault_account
    // ctx.accounts.user_vault_account.publickey = ctx.accounts.user_vault_account.key();
    ctx.accounts.user_vault_account.vault = ctx.accounts.vault.key();
    ctx.accounts.user_vault_account.token_account = ctx.accounts.token_account.key();
    ctx.accounts.user_vault_account.authority = ctx.accounts.user_account_authority.key();
    ctx.accounts.user_vault_account.user_account_bump = user_account_bump;

    ctx.accounts.user_vault_account.deposit_limit = limit;
    ctx.accounts.user_vault_account.deposit = 0;
    
    ctx.accounts.user_vault_account.user_stake.stake_amount = 0;
    ctx.accounts.user_vault_account.user_stake.cancel = false;
    ctx.accounts.user_vault_account.user_stake.stake_request_active = false;
    ctx.accounts.user_vault_account.user_stake.status = RequestStatus::Inactive;
    
    ctx.accounts.user_vault_account.user_unstake.unstake_amount = 0;
    ctx.accounts.user_vault_account.user_unstake.cancel = false;
    ctx.accounts.user_vault_account.user_unstake.unstake_request_active = false;
    ctx.accounts.user_vault_account.user_unstake.status = RequestStatus::Inactive;
    
    Ok(())
}