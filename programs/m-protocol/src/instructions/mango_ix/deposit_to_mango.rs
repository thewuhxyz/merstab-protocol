use crate::constants::*;
use crate::cpi;
use crate::state::*;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
// use mango::state::MangoAccount;

// use solana_program::program::invoke_signed;

#[derive(Accounts)]
// #[instruction(amount: u64)]
pub struct DepositToMango<'info> {
    #[account(
        mut,
        has_one=manager @ ErrorCode::NotAdmin
    )]
    pub vault: Account<'info, Vault>,

    /// CHECK: Mango account info
    pub mango_group: AccountInfo<'info>,
    
    /// CHECK: Mango account info
    #[account(mut)]
    pub mango_account: AccountInfo<'info>,
    
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
    
    #[account(
        mut, 
        token::mint=vault.mint,
        token::authority=vault_authority,
        address=vault.token_account @ ErrorCode::TokenAccountNotMatch
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

     /// CHECK: vault authority
    #[account(
        seeds=[
            vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref()
        ],
        bump=vault.vault_authority_bump
    )]
    pub vault_authority: AccountInfo<'info>,
    
    pub manager: Signer<'info>,
    
    /// CHECK: Mango account info
    pub mango_program_id: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

impl <'info> DepositToMango <'info> {
    
    fn deposit_to_mango_context(&self) -> CpiContext<'_, '_, '_, 'info, cpi::Deposit<'info>> {

        let cpi_accounts = cpi::Deposit {
            mango_account: self.mango_account.to_account_info().clone(),
            mango_cache: self.mango_cache.to_account_info().clone(),
            mango_group: self.mango_group.to_account_info().clone(),
            mango_program_id: self.mango_program_id.to_account_info().clone(),
            node_bank: self.mango_node_bank.to_account_info().clone(),
            owner: self.vault_authority.to_account_info().clone(),
            owner_token_account: self.vault_token_account.to_account_info().clone(),
            root_bank: self.mango_root_bank.to_account_info().clone(),
            vault: self.mango_vault.to_account_info().clone(),
            token_program: self.token_program.to_account_info().clone(),
        };
        let cpi_program = self.mango_program_id.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)

    }
}

pub fn handler(
    ctx: Context<DepositToMango>,
    amount: u64,
    // mango_account_owner_bump: u8,
) -> Result<()> {
    cpi::deposit(ctx.accounts.deposit_to_mango_context().with_signer(
        &[&[
            &ctx.accounts.vault.key().as_ref().to_owned(),
            VAULT_PDA_AUTHORITY_SEED.as_ref(),
            &[ctx.accounts.vault.vault_authority_bump],
        ]],
    ), amount)?;

    Ok(())
}

// pub fn handler(
//     ctx: Context<DepositToMango>,
//     amount: u64,
//     // mango_account_owner_bump: u8,
// ) -> Result<()> {
//     // ctx.accounts.vault.deposit_equity = amount;

//     let accounts = [
//         ctx.accounts.mango_group.clone(),
//         ctx.accounts.mango_account.clone(),
//         ctx.accounts.vault_authority.clone(),
//         ctx.accounts.mango_cache.clone(),
//         ctx.accounts.mango_root_bank.clone(),
//         ctx.accounts.mango_node_bank.clone(),
//         ctx.accounts.mango_vault.clone(),
//         ctx.accounts
//             .vault_token_account
//             .to_account_info()
//             .clone(),
//     ];
//     let result = mango::instruction::deposit(
//         &ctx.accounts.mango_program_id.key(),
//         &ctx.accounts.mango_group.key(),
//         &ctx.accounts.mango_account.key(),
//         &ctx.accounts.vault_authority.key(),
//         &ctx.accounts.mango_cache.key(),
//         &ctx.accounts.mango_root_bank.key(),
//         &ctx.accounts.mango_node_bank.key(),
//         &ctx.accounts.mango_vault.key(),
//         &ctx.accounts.vault_token_account.key(),
//         amount,
//     );
//     let instruction = match result {
//         Ok(is) => {
//             msg!("instruction was created successfully");
//             is
//         }
//         Err(error) => panic!("failed to deposit mango account: {:?}", error),
//     };

//     invoke_signed(
//         &instruction,
//         &accounts,
//         &[&[
//             &ctx.accounts.vault.key().as_ref().to_owned(),
//             VAULT_PDA_AUTHORITY_SEED.as_ref(),
//             &[ctx.accounts.vault.vault_authority_bump],
//         ]],
//     )?;
//     Ok(())
// }
