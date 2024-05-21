use crate::mango_ix;
use crate::state::*;
use crate::constants::*;
use crate::error::ErrorCode;

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};


#[derive(Accounts)]
pub struct ProcessStake<'info> {
    #[account(
        mut, 
        has_one=manager @ ErrorCode::NotAdmin
    )]
    pub vault: Box<Account<'info, Vault>>,

    #[account(
        mut,
        seeds=[
            user_vault_account.vault.key().as_ref(),
            user_vault_account.authority.key().as_ref(),
        ],
        bump=user_vault_account.user_account_bump,
        has_one=vault @ ErrorCode::VaultNotMatch,
        constraint=user_vault_account.user_stake.stake_request_active==true @ ErrorCode::NoRequestSent
    )]
    pub user_vault_account: Box<Account<'info, UserVaultAccount>>,

    /// CHECK: vault authority
     #[account(
        seeds=[
            vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref()
        ],
        bump=vault.vault_authority_bump
    )]
    pub vault_authority: UncheckedAccount<'info>,


    pub manager: Signer<'info>,

    #[account(
        mut, 
        token::mint=vault.mint,
        token::authority=vault_authority,
        address=user_vault_account.token_account @ ErrorCode::TokenAccountNotMatch,
        constraint = user_token_account.amount >= user_vault_account.user_stake.stake_amount @ ErrorCode::InsufficientBalance
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut, 
        token::mint=vault.mint,
        token::authority=vault_authority,
        address=vault.token_account @ ErrorCode::TokenAccountNotMatch
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// CHECK: Mango account info
    pub mango_group: AccountInfo<'info>,
    
    /// CHECK: Mango account info
    #[account(mut)]
    pub mango_account: AccountInfo<'info>,
    // pub mango_account: AccountLoader<'info, MangoAccount>,
    
    /// CHECK: Mango account info
    #[account(mut)]
    pub mango_cache: AccountInfo<'info>,
    
    /// CHECK: Mango account info
    #[account(mut)]
    pub mango_root_bank: AccountInfo<'info>,
    
    /// CHECK: Mango account info
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,
    
    /// CHECK: Mango account info
    #[account(mut)]
    pub mango_vault: AccountInfo<'info>,

    /// CHECK: Mango account info
    pub mango_program_id: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ProcessStake<'info> {
    // // * transfer usdc
    // fn usdc_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    //     let cpi_accounts = Transfer {
    //         from: self.user_token_account.to_account_info().clone(),
    //         to: self.vault_token_account.to_account_info().clone(),
    //         authority: self.vault_authority.to_account_info().clone(),
    //     };
    //     let cpi_program = self.token_program.to_account_info();
    //     CpiContext::new(cpi_program, cpi_accounts)
    // }

    fn deposit_to_mango_context(&self) -> CpiContext<'_, '_, '_, 'info, mango_ix::cpi::Deposit<'info>> {

        let cpi_accounts = mango_ix::cpi::Deposit {
            mango_account: self.mango_account.to_account_info().clone(),
            mango_cache: self.mango_cache.to_account_info().clone(),
            mango_group: self.mango_group.to_account_info().clone(),
            mango_program_id: self.mango_program_id.to_account_info().clone(),
            node_bank: self.mango_node_bank.to_account_info().clone(),
            owner: self.vault_authority.to_account_info().clone(),
            owner_token_account: self.user_token_account.to_account_info().clone(),
            root_bank: self.mango_root_bank.to_account_info().clone(),
            vault: self.mango_vault.to_account_info().clone(),
            token_program: self.token_program.to_account_info().clone(),
        };
        let cpi_program = self.mango_program_id.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)

    }

}

pub fn handler(ctx: Context<ProcessStake>) -> Result<()> {
    // * cancel stake
    if ctx.accounts.user_vault_account.user_stake.cancel {
        ctx.accounts.user_vault_account.user_stake.status = RequestStatus::Cancelled;
    } 
    
    // // * check if there is enough balance in token account
    // else if ctx.accounts.user_token_account.amount < ctx.accounts.user_vault_account.user_stake.stake_amount{
    //     ctx.accounts.user_vault_account.user_stake.status = RequestStatus::InsufficientBalance;
    // }
    
    // * process stake
    else {

        // let seed_signature = &[
        //     &ctx.accounts.vault.key().as_ref().to_owned(),
        //     &b"pdaauthority".as_ref()[..],
        //     &[ctx.accounts.vault.vault_authority_bump],
        // ];

        let amount: u64;
        
        if ctx.accounts.user_vault_account.user_stake.max {
            amount = ctx.accounts.user_token_account.amount
        } else {
            amount = ctx.accounts.user_vault_account.user_stake.stake_amount;
        }

        // token::transfer(
        //     ctx.accounts
        //         .usdc_transfer_context()
        //         .with_signer(&[&seed_signature[..]]),
        //     amount,
        // )?;

        mango_ix::cpi::deposit(ctx.accounts.deposit_to_mango_context().with_signer(
        &[&[
            &ctx.accounts.vault.key().as_ref().to_owned(),
            VAULT_PDA_AUTHORITY_SEED.as_ref(),
            &[ctx.accounts.vault.vault_authority_bump],
        ]],
    ), amount)?;
    
        ctx.accounts.user_vault_account.equity = ctx.accounts.user_vault_account.equity.checked_add(amount).unwrap();
        ctx.accounts.user_vault_account.user_total_stake = ctx.accounts.user_vault_account.user_total_stake.checked_add(amount).unwrap();
        ctx.accounts.vault.total_equity = ctx.accounts.vault.total_equity.checked_add(amount).unwrap();

        ctx.accounts.user_vault_account.user_pnl = ctx.accounts.user_vault_account.calculate_pnl();
        ctx.accounts.user_vault_account.user_stake.status = RequestStatus::Successful;
    }

    // ctx.accounts
    //     .user_vault_account
    //     .user_stake
    //     .cancel = false;
    ctx.accounts
        .user_vault_account
        .user_stake
        .stake_request_active = false;
    
        Ok(())
}
