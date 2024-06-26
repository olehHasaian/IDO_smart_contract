use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserState {
    // to avoid reinitialization attack
    pub is_initialized: u8,
    // user
    pub user: Pubkey,
    // totally staked amount
    pub total_staked_amount: u64,
    // total staked card count
    pub total_stake_card: u64,
    
    pub tier: u8,
    // reserved space
    pub reserved: [u128; 3]
}
