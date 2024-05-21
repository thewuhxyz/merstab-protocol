use crate::state::*;
use crate::error::ErrorCode;
use crate::constants::*;

use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use mango::state::{MangoAccount, MangoGroup, MangoCache};

#[derive(Accounts)]
pub struct UpdateVaultBalance<'info> {
    #[account(
        mut, 
        has_one=manager @ ErrorCode::NotAdmin
    )]
    pub vault: Account<'info, Vault>,

    #[account(
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

    /// CHECK: Mango Account CPI
    pub mango_account: AccountInfo<'info>,
    
    /// CHECK: Mango Account CPI
    #[account(mut)]
    pub mango_cache: AccountInfo<'info>,
    
    /// CHECK: Mango Account CPI    
    pub mango_program_id: AccountInfo<'info>,
    
    /// CHECK: Mango Account CPI    
    pub mango_group: AccountInfo<'info>,
   
    /// CHECK: Mango Account CPI    
    pub mango_root_bank: AccountInfo<'info>,

    pub manager: Signer<'info>,
}


impl<'info> UpdateVaultBalance<'info> {

    pub fn get_mango_balance (&self) -> u64 {
        let mango_group = MangoGroup::load_checked(
            &self.mango_group, 
            self.mango_program_id.key
        ).unwrap();

        // let acct = &self.mango_account;
        let mango_account =
            MangoAccount::load_mut_checked(
                &self.mango_account, 
                self.mango_program_id.key, 
                &self.mango_group.key
            ).unwrap();
        
        let token_index = mango_group
            .find_root_bank_index(self.mango_root_bank.key).unwrap(); // todo: change this error

        // let group = &mango_group;

        let mango_cache = MangoCache::load_checked(
            &self.mango_cache, 
            self.mango_program_id.key, 
            &mango_group
        ).unwrap();

        let root_bank_cache = &mango_cache.root_bank_cache[token_index];

        let native_deposit = mango_account.get_native_deposit(root_bank_cache, token_index).unwrap();

        native_deposit.checked_floor().unwrap().to_num::<u64>()

    }
}

pub fn handler(
    ctx: Context<UpdateVaultBalance>, 
    // new_balance: u64
) -> Result<()> {
    ctx.accounts.vault.previous_total_equity = ctx.accounts.vault.total_equity;
    
    let new_balance = ctx.accounts.get_mango_balance();
    
    // todo: operating costs will be debited here
    
    ctx.accounts.vault.total_equity = new_balance;
    ctx.accounts.vault.total_equity_before_settlements = new_balance;
    
    ctx.accounts.vault.day_pnl = ctx.accounts.vault.calculate_pnl_ratio();
    Ok(())
}
