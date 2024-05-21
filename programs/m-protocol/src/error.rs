use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Admin Signer Only")]
    NotAdmin = 300, //300, 0x12c

    #[msg("Vault PDA authority bump seed does not match the supplied bump")]
    BumpNotMatch,
    
    #[msg("Vault does not match")]
    VaultNotMatch,
    
    #[msg("Wrong User Account Authority Provided")]
    WrongUserAccountAuthority,
    
    #[msg("Wrong Mint Provided")]
    WrongMintProvided,
    
    #[msg("Token Account Not Match")]
    TokenAccountNotMatch,
   
    #[msg("Stake Request Already Active")]
    StakeRequestActive,
    
    #[msg("Unstake Request Already Active")]
    UnstakeRequestActive,
    
    #[msg("No Request Sent")]
    NoRequestSent,

    #[msg("Max Request Limit Hit")]
    MaxRequestLimit,
    
    #[msg("Max Deposit Limit Exceeded")]
    MaxDepositLimit,
    
    #[msg("Max Vault Deposit Limit Exceeded")]
    MaxVaultLimit,
    
    #[msg("Insufficient Balance")]
    InsufficientBalance,

    #[msg("the stake mint does not match")]
    StakeMintMismatch, //308, 0x134
}
