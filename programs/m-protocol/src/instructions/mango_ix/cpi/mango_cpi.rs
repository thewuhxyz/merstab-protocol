use anchor_lang::prelude::*;
use mango::state::MAX_PAIRS;


pub fn create_mango_account<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateMangoAccount<'info>>,
    account_num: u64,
) -> Result<()> {
    let ix = mango::instruction::create_mango_account(
        ctx.accounts.mango_program_id.key,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.system_prog.key,
        ctx.accounts.payer.key,
        account_num,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn close_mango_account<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CloseMangoAccount<'info>>,
) -> Result<()> {
    let ix = mango::instruction::close_mango_account(
        ctx.accounts.mango_program_id.key,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn deposit<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Deposit<'info>>,
    quantity: u64,
) -> Result<()> {
    let ix = mango::instruction::deposit(
        ctx.accounts.mango_program_id.key,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.mango_cache.key,
        ctx.accounts.root_bank.key,
        ctx.accounts.node_bank.key,
        ctx.accounts.vault.key,
        ctx.accounts.owner_token_account.key,
        quantity,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn withdraw<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Withdraw<'info>>,
    quantity: u64,
    allow_borrow: bool,
) -> Result<()> {
    let remaining_accounts_iter = ctx.remaining_accounts.iter();
    let mut open_orders = vec![Pubkey::default(); MAX_PAIRS];
    remaining_accounts_iter.for_each(|ai| open_orders.push(*ai.key));
    let ix = mango::instruction::withdraw(
        ctx.accounts.mango_program_id.key,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.mango_cache.key,
        ctx.accounts.root_bank.key,
        ctx.accounts.node_bank.key,
        ctx.accounts.vault.key,
        ctx.accounts.token_account.key,
        ctx.accounts.signer.key,
        open_orders.as_slice(),
        quantity,
        allow_borrow,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn set_delegate<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SetDelegate<'info>>,
) -> Result<()> {
    let ix = mango::instruction::set_delegate(
        ctx.accounts.mango_program_id.key,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.delegate_pubkey.key,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct CreateMangoAccount<'info> {
    /// CHECK: Mango CPI
    pub mango_group: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub owner: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub system_prog: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub payer: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_program_id: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct CloseMangoAccount<'info> {
    /// CHECK: Mango CPI
    pub mango_group: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub owner: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// CHECK: Mango CPI
    pub mango_group: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub owner: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_cache: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub root_bank: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub node_bank: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub vault: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub owner_token_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub token_program: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_program_id: AccountInfo<'info>,
}

/// To reference OpenOrders, add them to the accounts [0-MAX_PAIRS] of the
/// CpiContext's `remaining_accounts` Vec.
#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// CHECK: Mango CPI
    pub mango_group: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub owner: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_cache: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub root_bank: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub node_bank: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub vault: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub token_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub signer: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub token_program: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetDelegate<'info> {
    /// CHECK: Mango CPI
    pub mango_group: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub owner: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub delegate_pubkey: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_program_id: AccountInfo<'info>,
}