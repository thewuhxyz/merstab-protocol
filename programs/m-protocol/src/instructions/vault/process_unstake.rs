use crate::mango_ix;
use crate::state::*;
use crate::constants::*;
use crate::error::ErrorCode;

use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount };

#[derive(Accounts)]
pub struct ProcessUnstake<'info> {
    #[account(mut, has_one=manager)]
    pub vault: Box<Account<'info, Vault>>,

    #[account(
        mut, 
        seeds=[
            user_vault_account.vault.key().as_ref(),
            user_vault_account.authority.key().as_ref(),
        ],
        bump=user_vault_account.user_account_bump,
        has_one=vault @ ErrorCode::VaultNotMatch,
        constraint=user_vault_account.user_unstake.unstake_request_active==true @ ErrorCode::NoRequestSent,
        constraint = user_vault_account.equity >= user_vault_account.user_unstake.unstake_amount @ ErrorCode::InsufficientBalance
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

     ///CHECK: Mango Account Info
    #[account(mut)]
    pub mango_group: AccountInfo<'info>,
    
    ///CHECK: Mango Account Info
    #[account(
        mut,
        seeds=[
            mango_group.key().as_ref(),
            vault_authority.key().as_ref(),
            &vault.mango_account_num.to_le_bytes() // todo: might change to vault.vault_authority_bump to save space
            // &vault.vault_authority_bump.to_le_bytes() 
            ],
        bump=vault.mango_account_bump,
        seeds::program=mango_program_id.key()
    )]
    pub mango_account: AccountInfo<'info>,
    
    ///CHECK: Mango Account Info
    pub mango_cache: AccountInfo<'info>,
    
    ///CHECK: Mango Account Info
    pub mango_root_bank: AccountInfo<'info>,
    
    ///CHECK: Mango Account Info
    #[account(mut)]
    pub mango_node_bank: AccountInfo<'info>,
    
    ///CHECK: Mango Account Info
    #[account(mut)]
    pub mango_vault: AccountInfo<'info>,


    pub manager: Signer<'info>,

    #[account(
        mut, 
        // constraint = user_vault_usdc_token_account.mint == user_ata.mint,
        token::mint=vault.mint,
        token::authority=vault_authority,
        address=user_vault_account.token_account @ ErrorCode::TokenAccountNotMatch,

    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut, 
        // constraint = user_vault_usdc_token_account.mint == user_ata.mint,
        token::mint=vault.mint,
        token::authority=vault_authority,
        address=vault.token_account
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    ///CHECK: Mango Account Info
    pub signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    ///CHECK: Mango Account Info
    pub mango_program_id: AccountInfo<'info>,

}

impl<'info> ProcessUnstake<'info> {
    // * transfer usdc
    // fn usdc_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {        
    //     let cpi_accounts = Transfer {
    //         from: self.vault_token_account.to_account_info().clone(),
    //         to: self.user_token_account.to_account_info().clone(),
    //         authority: self.vault_authority.to_account_info().clone(),
    //     };
    //     let cpi_program = self.token_program.to_account_info();
    //     CpiContext::new(cpi_program, cpi_accounts)
    // }

    fn withdraw_from_mango_context(&self) -> CpiContext<'_, '_, '_, 'info, mango_ix::cpi::Withdraw<'info>> {

        let cpi_accounts = mango_ix::cpi::Withdraw {
            mango_account: self.mango_account.to_account_info().clone(),
            mango_cache: self.mango_cache.to_account_info().clone(),
            mango_group: self.mango_group.to_account_info().clone(),
            mango_program_id: self.mango_program_id.to_account_info().clone(),
            node_bank: self.mango_node_bank.to_account_info().clone(),
            owner: self.vault_authority.to_account_info().clone(),
            root_bank: self.mango_root_bank.to_account_info().clone(),
            signer: self.signer.to_account_info().clone(),
            token_account: self.user_token_account.to_account_info().clone(),
            vault: self.mango_vault.to_account_info().clone(),
            token_program: self.token_program.to_account_info().clone(),
        };
        let cpi_program = self.mango_program_id.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)

    }

}

pub fn handler(ctx: Context<ProcessUnstake>) -> Result<()> {
    // * cancel unstake

    if ctx.accounts.user_vault_account.user_unstake.cancel {
        ctx.accounts.user_vault_account.user_unstake.status = RequestStatus::Cancelled;
    }
    // * process unstake
    else {
        // let seed_signature = &[
        //     &ctx.accounts.vault.key().as_ref().to_owned(),
        //     &b"pdaauthority".as_ref()[..],
        //     &[ctx.accounts.vault.vault_authority_bump],
        // ];

        let amount:u64;

        if ctx.accounts.user_vault_account.user_unstake.max {
            amount = ctx.accounts.user_vault_account.equity;
        } else {
            amount = ctx.accounts.user_vault_account.user_unstake.unstake_amount;
        }

        mango_ix::cpi::withdraw(ctx.accounts.withdraw_from_mango_context().with_signer(
            &[&[
                &ctx.accounts.vault.key().as_ref().to_owned(),
                VAULT_PDA_AUTHORITY_SEED.as_ref(),
                &[ctx.accounts.vault.vault_authority_bump],
            ]],
        ), amount, false)?;

        // token::transfer(
        //     ctx.accounts
        //         .usdc_transfer_context()
        //         .with_signer(&[&seed_signature[..]]),
        //     amount,
        // )?;
        
        ctx.accounts.user_vault_account.equity = ctx.accounts.user_vault_account.equity.checked_sub(amount).unwrap(); 
        ctx.accounts.user_vault_account.user_total_unstake = ctx.accounts.user_vault_account.user_total_unstake.checked_add(amount).unwrap() ; 
        ctx.accounts.vault.total_equity = ctx.accounts.vault.total_equity.checked_sub(amount).unwrap(); 
        
        ctx.accounts.user_vault_account.user_unstake.status = RequestStatus::Successful;

        if ctx.accounts.user_vault_account.equity == 0 {
            ctx.accounts.user_vault_account.refresh_stats()
        }
    }

    // ctx.accounts.user_vault_account.user_unstake.cancel = false;
    ctx.accounts
        .user_vault_account
        .user_unstake
        .unstake_request_active = false;

    Ok(())
}
