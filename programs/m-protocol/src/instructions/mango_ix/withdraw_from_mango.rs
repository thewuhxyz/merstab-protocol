use crate::constants::*;
use crate::cpi;
use crate::cpi::*;
use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
// use solana_program::program::invoke_signed;

#[derive(Accounts)]
// #[instruction(mango_account_owner_bump: u8)]
pub struct WithdrawFromMango<'info> {
    #[account(
        mut,
        has_one=manager
    )]
    pub vault: Account<'info, Vault>,

    ///CHECK: Mango Account Info
    #[account(mut)]
    pub mango_group: AccountInfo<'info>,
    
    ///CHECK: Mango Account Info
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
    pub vault_authority: AccountInfo<'info>, //
    
    ///CHECK: Mango Account Info
    pub signer: AccountInfo<'info>,
    
    pub manager: Signer<'info>,

    pub token_program: Program<'info, Token>,

    ///CHECK: Mango Account Info
    pub mango_program_id: AccountInfo<'info>,
}

impl <'info> WithdrawFromMango <'info> {
    
    fn withdraw_from_mango_context(&self) -> CpiContext<'_, '_, '_, 'info, Withdraw<'info>> {

        let cpi_accounts = cpi::Withdraw {
            mango_account: self.mango_account.to_account_info().clone(),
            mango_cache: self.mango_cache.to_account_info().clone(),
            mango_group: self.mango_group.to_account_info().clone(),
            mango_program_id: self.mango_program_id.to_account_info().clone(),
            node_bank: self.mango_node_bank.to_account_info().clone(),
            owner: self.vault_authority.to_account_info().clone(),
            root_bank: self.mango_root_bank.to_account_info().clone(),
            signer: self.signer.to_account_info().clone(),
            token_account: self.vault_token_account.to_account_info().clone(),
            vault: self.mango_vault.to_account_info().clone(),
            token_program: self.token_program.to_account_info().clone(),

        };
        let cpi_program = self.mango_program_id.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)

    }
}

pub fn handler(
    ctx: Context<WithdrawFromMango>,
    amount: u64,
    // mango_account_owner_bump: u8,
) -> Result<()> {

    // let mango_account_pda = Pubkey::create_program_address(
    //     &[
    //         ctx.accounts.mango_group.key().as_ref(),
    //         ctx.accounts.vault_authority.key().as_ref(),
    //         &ctx.accounts.vault.mango_account_num.to_le_bytes(),
    //         &[ctx.accounts.vault.mango_account_bump],
    //     ],
    //     &ctx.accounts.mango_program_id.key().clone(),
    // )
    // .unwrap_or_default();


    cpi::withdraw(ctx.accounts.withdraw_from_mango_context().with_signer(
        &[&[
            &ctx.accounts.vault.key().as_ref().to_owned(),
            VAULT_PDA_AUTHORITY_SEED.as_ref(),
            &[ctx.accounts.vault.vault_authority_bump],
        ]],
    ), amount, false)?;


    Ok(())
}

// pub fn handler(
//     ctx: Context<WithdrawFromMango>,
//     amount: u64,
//     // mango_account_owner_bump: u8,
// ) -> Result<()> {

//     let mango_account_pda = Pubkey::create_program_address(
//         &[
//             ctx.accounts.mango_group.key().as_ref(),
//             ctx.accounts.vault_authority.key().as_ref(),
//             &ctx.accounts.vault.mango_account_num.to_le_bytes(),
//             &[ctx.accounts.vault.mango_account_bump],
//         ],
//         &ctx.accounts.mango_program_id.key().clone(),
//     )
//     .unwrap_or_default();

//     assert_eq!(ctx.accounts.mango_account.key(), mango_account_pda);

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
//         ctx.accounts.signer.clone(),
//     ];

//     let mut open_orders: [Pubkey; 15] = [ctx.accounts.mango_account.key(); 15];
//     for i in 0..ctx.remaining_accounts.len() {
//         open_orders[i] = ctx.remaining_accounts[i].key();
//     }

//     let result = mango::instruction::withdraw(
//         &ctx.accounts.mango_program_id.key(),
//         &ctx.accounts.mango_group.key(),
//         &ctx.accounts.mango_account.key().clone(),
//         &ctx.accounts.vault_authority.key(),
//         &ctx.accounts.mango_cache.key(),
//         &ctx.accounts.mango_root_bank.key(),
//         &ctx.accounts.mango_node_bank.key(),
//         &ctx.accounts.mango_vault.key(),
//         &ctx.accounts.vault_token_account.key(),
//         &ctx.accounts.signer.key(),
//         &open_orders,
//         amount,
//         true,
//     );
//     let instruction = match result {
//         Ok(is) => {
//             msg!("instruction was created successfully");
//             is
//         }
//         Err(error) => panic!("failed to withdraw from mango account: {:?}", error),
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
