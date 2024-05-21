// use crate::CloseMangoAccountCpi;
use crate::constants::*;
use crate::cpi;
use crate::state::*;
use anchor_lang::prelude::*;
// use solana_program::program::invoke_signed;

#[derive(Accounts)]
// #[instruction(mango_account_owner_bump: u8)]
pub struct DelegateMangoAccount<'info> {
    #[account(
        has_one=manager
    )]
    pub vault: Account<'info, Vault>,
    
    /// CHECK: Mango acoount info
    #[account(mut)]
    pub mango_group: AccountInfo<'info>,
    
    /// CHECK: Mango acoount info
    #[account(
        mut,
        seeds=[
            mango_group.key().as_ref(),
            vault_authority.key().as_ref(),
            &vault.mango_account_num.to_le_bytes() 
        ],
        bump=vault.mango_account_bump,
        seeds::program=mango_program_id.key()
    )]
    pub mango_account: AccountInfo<'info>,

    /// CHECK: vault authority
    #[account(
        seeds=[
            vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref()
        ],
        bump=vault.vault_authority_bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    /// CHECK: Mango acoount info
    pub delegate_pubkey: UncheckedAccount<'info>,

    pub manager: Signer<'info>,

    /// CHECK: Mango acoount info
    pub mango_program_id: AccountInfo<'info>,
}

impl <'info> DelegateMangoAccount <'info> {
    
    fn delegate_mango_account_context(&self) -> CpiContext<'_, '_, '_, 'info, cpi::SetDelegate<'info>> {

        let cpi_accounts = cpi::SetDelegate {
            mango_account: self.mango_account.to_account_info().clone(),
            mango_group: self.mango_group.to_account_info().clone(),
            mango_program_id: self.mango_program_id.to_account_info().clone(),
            owner: self.vault_authority.to_account_info().clone(),
            delegate_pubkey: self.delegate_pubkey.to_account_info().clone()
        };
        let cpi_program = self.mango_program_id.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)

    }
}

pub fn handler(ctx: Context<DelegateMangoAccount>) -> Result<()> {
    
    cpi::mango_cpi::set_delegate(ctx.accounts.delegate_mango_account_context().with_signer(
        &[&[
            ctx.accounts.vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref(),
            &[ctx.accounts.vault.vault_authority_bump],
        ]],
    ))?;

    Ok(())
}
