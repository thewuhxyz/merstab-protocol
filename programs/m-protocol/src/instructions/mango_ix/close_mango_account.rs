use crate::constants::*;
use crate::cpi;
use crate::state::*;
use anchor_lang::prelude::*;
// use solana_program::program::invoke_signed;

#[derive(Accounts)]
pub struct CloseAccountOnMango<'info> {
    #[account(
        mut,
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
        seeds::program=mango_program_id.key(),        
    )]
    pub mango_account: AccountInfo<'info>,

    /// CHECK: vault authority
    #[account(
        mut,
        seeds=[
            vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref()
        ],
        bump=vault.vault_authority_bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub manager: Signer<'info>,

    /// CHECK: Mango acoount info
    pub mango_program_id: AccountInfo<'info>,
}

impl <'info> CloseAccountOnMango <'info> {
    
    fn close_mango_account_context(&self) -> CpiContext<'_, '_, '_, 'info, cpi::CloseMangoAccount<'info>> {

        let cpi_accounts = cpi::CloseMangoAccount {
            mango_account: self.mango_account.to_account_info().clone(),
            mango_group: self.mango_group.to_account_info().clone(),
            mango_program_id: self.mango_program_id.to_account_info().clone(),
            owner: self.vault_authority.to_account_info().clone(), 
        };
        let cpi_program = self.mango_program_id.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)

    }
}

pub fn handler(ctx: Context<CloseAccountOnMango>) -> Result<()> {
    
    cpi::close_mango_account(ctx.accounts.close_mango_account_context().with_signer(
        &[&[
            ctx.accounts.vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref(),
            &[ctx.accounts.vault.vault_authority_bump],
        ]],
    ))?;

    Ok(())
}

// pub fn handler(ctx: Context<CloseMangoAccount>) -> Result<()> {
//     let accounts = [
//         ctx.accounts.mango_group.clone(),
//         ctx.accounts.mango_account.clone(),
//         ctx.accounts.vault_authority.clone(),
//     ];
//     let result = mango::instruction::close_mango_account(
//         &ctx.accounts.mango_program_id.key(),
//         &ctx.accounts.mango_group.key(),
//         &ctx.accounts.mango_account.key(),
//         &ctx.accounts.vault_authority.key(),
//     );
//     let instruction = match result {
//         Ok(is) => {
//             msg!("instruction was created successfully");
//             is
//         }
//         Err(error) => panic!("failed to close mango account: {:?}", error),
//     };

//     invoke_signed(
//         &instruction,
//         &accounts,
//         &[&[
//             ctx.accounts.vault.key().as_ref(),
//             VAULT_PDA_AUTHORITY_SEED.as_ref(),
//             &[ctx.accounts.vault.vault_authority_bump],
//         ]],
//     )?;
//     Ok(())
// }
