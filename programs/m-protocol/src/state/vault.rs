use crate::constants::*;
use std::str::FromStr;

use anchor_lang::prelude::*;
use rust_decimal::prelude::ToPrimitive;

#[account]
pub struct Vault {
    pub publickey: Pubkey, // 32
    pub manager: Pubkey, // 32
    pub name: String, // 4 + 16
    
    pub limit: u64, // 8
    pub deposit: u64, // 8

    pub total_equity: u64, // 8
    pub previous_total_equity: u64, // 8
    pub total_equity_before_settlements: u64, // 8
    pub day_pnl: f64, // 8

    pub token_account: Pubkey, // 32 // * vault usdc main token account
    pub mint: Pubkey, // 32          // * vault usdc token mint

    pub stake_request_account: Pubkey, // 32
    pub unstake_request_account: Pubkey, // 32

    pub vault_bump: u8, // 1  // * Vault bump
    pub vault_authority_bump: u8, // 1  // * Vault authority bump

    pub mango_account_bump: u8, // 1
    pub mango_account_num: u64, // 8
}

impl Vault {
    pub fn manager() -> Pubkey {
        Pubkey::from_str(MANAGER_PUBKEY).unwrap()
    }

    pub fn calculate_daily_pnl(&self) -> i128 {
        (self.total_equity as i128) - (self.previous_total_equity as i128)
    }

    pub fn calculate_pnl_ratio(&self) -> f64 {
        if self.previous_total_equity == 0 {
            return 0_f64;
        }
        self.calculate_daily_pnl().to_f64().unwrap() / self.previous_total_equity.to_f64().unwrap()
    }

    pub const LEN: usize = (
        (8 * 5) + // u64
        8 + // day_pnl
        (4 + 16) + // 16 characters max string
        (1 * 2) + // u8
        (32 * 6) // pubkey
    );
}

#[account]
pub struct UserVaultAccount {
    // pub publickey: Pubkey,
    pub vault: Pubkey,
    pub deposit_limit: u64,
    pub deposit: u64,
    pub withdrawal: u64,
    pub equity: u64,
    pub user_total_stake: u64,
    pub user_total_unstake: u64,
    pub user_pnl: f64,
    pub authority: Pubkey,
    pub token_account: Pubkey,
    pub user_stake: UserStake,
    pub user_unstake: UserUnstake,
    pub last_trade_stat: LastTradeStat,
    pub user_account_bump: u8
}

impl UserVaultAccount {
    pub fn calculate_pnl(&self) -> f64 {
        if self.user_total_stake == 0 {
            return 0_f64;
        }
        (self.user_total_unstake.to_f64().unwrap() + self.equity.to_f64().unwrap()
            - self.user_total_stake.to_f64().unwrap())
            / self.user_total_stake.to_f64().unwrap()
    }

    pub fn calclulate_realised_pnl(&self) -> f64 {
        if self.last_trade_stat.user_total_stake == 0 {
            return 0_f64;
        }
        (self.last_trade_stat.user_total_unstake.to_f64().unwrap()
            - self.last_trade_stat.user_total_stake.to_f64().unwrap())
            / self.last_trade_stat.user_total_stake.to_f64().unwrap()
    }

    pub fn refresh_stats(&mut self) {
        if self.equity == 0 {
            self.last_trade_stat.user_total_stake = self.user_total_stake;
            self.last_trade_stat.user_total_unstake = self.user_total_unstake;

            self.user_total_stake = 0_u64;
            self.user_total_unstake = 0_u64;
            self.user_pnl = 0_f64;

            self.last_trade_stat.user_realised_pnl = self.calclulate_realised_pnl()
        }
    }
}

#[account(zero_copy)]
pub struct StakeReq {
    pub vault: Pubkey,
    pub max_requests: u32,
    pub count: u32,                     // 4
    pub orders: [Pubkey; MAX_REQUESTS], // 32 * 1000 + 1
}

#[account(zero_copy)]
pub struct UnstakeReq {
    pub vault: Pubkey,
    pub max_requests: u32,
    pub count: u32,                     // 4
    pub orders: [Pubkey; MAX_REQUESTS], // 32 *1000 + 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub struct UserStake {
    pub stake_amount: u64,
    pub stake_request_active: bool,
    pub max: bool,
    pub cancel: bool,
    pub status: RequestStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub struct UserUnstake {
    pub unstake_amount: u64,
    pub unstake_request_active: bool,
    pub max: bool,
    pub cancel: bool,
    pub status: RequestStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub struct LastTradeStat {
    pub user_total_stake: u64,
    pub user_total_unstake: u64,
    pub user_realised_pnl: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum RequestStatus {
    Inactive = 0,
    Pending = 1,
    Successful = 2,
    Unsuccessful = 3,
    Cancelled = 4,
    InsufficientBalance = 5,
}
