use crate::state::*;
use crate::error::ErrorCode;
use crate::constants::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};


#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateVault <'info> {
    // vault account
    #[account(
        init, 
        payer=manager,
        seeds=[
            name.as_ref(),
            VAULT_SEED.as_ref()
            ],
        bump, 
        space=8+512, // todo: adjust space later
    )]
    pub vault: Box<Account<'info, Vault>>,


    #[account(zero)]
    pub stake_req: AccountLoader<'info, StakeReq>,
    

    #[account(zero)]
    pub unstake_req: AccountLoader<'info, UnstakeReq>,

    //* account that trades for the vault e.g on mango markets
    #[account(
        mut, 
        constraint = *manager.key==Vault::manager() @ ErrorCode::NotAdmin
    )]
    pub manager: Signer<'info>,
    
    // usdc token account to hold usdc tokens
    #[account(
        init,
        payer=manager,
        seeds=[
            vault.key().as_ref(),
            USDC_SEED.as_ref()
            ],
        bump,
        token::mint = token_mint,
        token::authority = vault_pda_authority,
        )]
    pub token_account: Account<'info, TokenAccount>,
    
    // * seeds=[vault.key().as_ref(), b"pdaauthority".as_ref()]
    ///CHECK: pda authority
    #[account(
        seeds=[
            vault.key().as_ref(),
            VAULT_PDA_AUTHORITY_SEED.as_ref()
            ],
        bump,
    )]
    pub  vault_pda_authority: UncheckedAccount<'info>,
    
    pub token_mint: Account<'info, Mint>,

    pub rent: Sysvar<'info, Rent>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>
}

pub fn handler (
    ctx: Context<CreateVault>, 
    name: String,
    limit: u64,
    vault_bump: u8,
    vault_pda_bump: u8, 
) -> Result<()> {

    let vault_key = Pubkey::create_program_address(
        &[
            name.as_ref(), 
            VAULT_SEED.as_ref(),
            &[vault_bump]
            ], 
            ctx.program_id
        ).unwrap_or_default();
    
    if vault_key != ctx.accounts.vault.key() {
        return err!(ErrorCode::BumpNotMatch)
    }
    
    let vault_pda = Pubkey::create_program_address(
        &[
            ctx.accounts.vault.key().as_ref(), 
            b"pdaauthority".as_ref(), 
            &[vault_pda_bump]
            ], 
            ctx.program_id
        ).unwrap_or_default();
    
    if vault_pda != ctx.accounts.vault_pda_authority.key() {
        return err!(ErrorCode::BumpNotMatch)
    } 

    let mut stake_req = ctx.accounts.stake_req.load_init()?;
    stake_req.vault=ctx.accounts.vault.key();
    stake_req.max_requests=1000;
    
    let mut unstake_req = ctx.accounts.unstake_req.load_init()?;
    unstake_req.vault=ctx.accounts.vault.key();
    unstake_req.max_requests=1000;
    
    ctx.accounts.vault.publickey = ctx.accounts.vault.key();
    ctx.accounts.vault.manager = ctx.accounts.manager.key();
    ctx.accounts.vault.name = name;
    ctx.accounts.vault.vault_bump = vault_bump;
    ctx.accounts.vault.vault_authority_bump = vault_pda_bump;
    ctx.accounts.vault.limit = limit;
    ctx.accounts.vault.total_equity = 0;
    ctx.accounts.vault.total_equity_before_settlements = 0;
    ctx.accounts.vault.previous_total_equity = 0;
    ctx.accounts.vault.token_account = ctx.accounts.token_account.key();
    ctx.accounts.vault.stake_request_account = ctx.accounts.stake_req.key();
    ctx.accounts.vault.unstake_request_account = ctx.accounts.unstake_req.key();
    ctx.accounts.vault.mint = ctx.accounts.token_mint.key();
    // ctx.accounts.vault.vault_pda_authority = ctx.accounts.vault_pda_authority.key();

    Ok(())
}