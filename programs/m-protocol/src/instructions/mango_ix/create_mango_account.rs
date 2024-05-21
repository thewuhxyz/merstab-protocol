use crate::{constants::*, cpi};
use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
// use solana_program::program::invoke_signed;

#[derive(Accounts)]
#[instruction(account_num:u64, mango_account_bump:u8)]
pub struct CreateAccountOnMango<'info> {
    #[account(
        mut, 
        has_one=manager @ ErrorCode::NotAdmin
    )]
    pub vault: Account<'info, Vault>,

     /// CHECK: vault authority
    #[account(
        mut,
        seeds=[
            vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref()
        ],
        bump=vault.vault_authority_bump
    )]
    pub vault_authority: AccountInfo<'info>,
            
    /// CHECK: Mango account info
    #[account(
        mut,
        seeds=[
            mango_group_ai.key().as_ref(),
            vault_authority.key().as_ref(),
            &account_num.to_le_bytes() 
        ],
        bump=mango_account_bump,
        seeds::program=mango_program_id.key()
    )]
    pub unverified_mango_account_pda: AccountInfo<'info>, // ^^ as above
    
    #[account(mut)]
    pub manager: Signer<'info>,
    
    /// CHECK: Mango account info
    pub mango_program_id: AccountInfo<'info>,
    
    /// CHECK: Mango account info
    #[account(mut)]
    pub mango_group_ai: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

impl <'info> CreateAccountOnMango <'info> {
    
    fn create_mango_account_context(&self) -> CpiContext<'_, '_, '_, 'info, cpi::CreateMangoAccount<'info>> {

        let cpi_accounts = cpi::CreateMangoAccount {
            mango_account: self.unverified_mango_account_pda.to_account_info().clone(),
            mango_group: self.mango_group_ai.to_account_info().clone(),
            mango_program_id: self.mango_program_id.to_account_info().clone(),
            owner: self.vault_authority.to_account_info().clone(),
            payer: self.manager.to_account_info().clone(),
            system_prog: self.system_program.to_account_info().clone()
        };
        let cpi_program = self.mango_program_id.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)

    }
}

pub fn handler(
    ctx: Context<CreateAccountOnMango>,
    // mango_account_owner_bump: u8,
    account_num: u64,
    mango_account_bump: u8,
) -> Result<()> {
    let pda = Pubkey::create_program_address(
        &[
            ctx.accounts.vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref(),
            &[ctx.accounts.vault.vault_authority_bump],
        ],
        ctx.program_id,
    )
    .unwrap_or_default();

    if pda != ctx.accounts.vault_authority.key() {
        return err!(ErrorCode::BumpNotMatch);
    }

    // let account_num = ctx.accounts.vault.vault_authority_bump as u64;

    let mango_account_pda = Pubkey::create_program_address(
        &[
            ctx.accounts.mango_group_ai.key().as_ref(),
            ctx.accounts.vault_authority.key().as_ref(),
            &account_num.to_le_bytes(),
            &[mango_account_bump],
        ],
        &ctx.accounts.mango_program_id.key().clone(),
    )
    .unwrap_or_default();

    if mango_account_pda != ctx.accounts.unverified_mango_account_pda.key() {
        return err!(ErrorCode::BumpNotMatch);
    }

    cpi::create_mango_account(ctx.accounts.create_mango_account_context().with_signer(
        &[&[
            &ctx.accounts.vault.key().as_ref().to_owned(),
            VAULT_PDA_AUTHORITY_SEED.as_ref(),
            &[ctx.accounts.vault.vault_authority_bump],
        ]],
    ), account_num)?;
    

    ctx.accounts.vault.mango_account_bump = mango_account_bump;
    ctx.accounts.vault.mango_account_num = account_num;

    Ok(())
}

// pub fn handler(
//     ctx: Context<CreateMangoAccount>,
//     // mango_account_owner_bump: u8,
//     account_num: u64,
//     mango_account_bump: u8,
// ) -> Result<()> {
//     let pda = Pubkey::create_program_address(
//         &[
//             ctx.accounts.vault.key().as_ref(),
//             VAULT_PDA_AUTHORITY_SEED.as_ref(),
//             &[ctx.accounts.vault.vault_authority_bump],
//         ],
//         ctx.program_id,
//     )
//     .unwrap_or_default();

//     if pda != ctx.accounts.vault_authority.key() {
//         return err!(ErrorCode::BumpNotMatch);
//     }

//     // let account_num = ctx.accounts.vault.vault_authority_bump as u64;

//     let mango_account_pda = Pubkey::create_program_address(
//         &[
//             ctx.accounts.mango_group_ai.key().as_ref(),
//             ctx.accounts.vault_authority.key().as_ref(),
//             &account_num.to_le_bytes(),
//             &[mango_account_bump],
//         ],
//         &ctx.accounts.mango_program_id.key().clone(),
//     )
//     .unwrap_or_default();

//     if mango_account_pda != ctx.accounts.unverified_mango_account_pda.key() {
//         return err!(ErrorCode::BumpNotMatch);
//     }

//     assert_eq!(
//         ctx.accounts.unverified_mango_account_pda.key(),
//         mango_account_pda
//     );

//     let accounts = [
//         ctx.accounts.mango_group_ai.clone(),
//         ctx.accounts.unverified_mango_account_pda.clone(),
//         ctx.accounts.vault_authority.clone(),
//         ctx.accounts.system_program.to_account_info().clone(),
//         ctx.accounts.manager.to_account_info().clone(),
//     ];
//     let result = mango::instruction::create_mango_account(
//         &ctx.accounts.mango_program_id.key().clone(),
//         &ctx.accounts.mango_group_ai.key().clone(),
//         &ctx.accounts.unverified_mango_account_pda.key().clone(),
//         &ctx.accounts.vault_authority.key().clone(),
//         &ctx.accounts.system_program.key().clone(),
//         &ctx.accounts.manager.key().clone(),
//         account_num as u64,
//     );
//     let instruction = match result {
//         Ok(is) => {
//             msg!("instruction was created successfully");
//             is
//         }
//         Err(error) => panic!("failed to create mango account: {:?}", error),
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

//     ctx.accounts.vault.mango_account_bump = mango_account_bump;
//     ctx.accounts.vault.mango_account_num = account_num;

//     Ok(())
// }
