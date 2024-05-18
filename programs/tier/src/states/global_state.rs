use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    // to avoid reinitialization attack
    pub is_initialized: u8,
    // admin
    pub authority: Pubkey,
    // treasury
    pub treasury: Pubkey,
    // token for staking
    pub yoiu_token_mint: Pubkey,
    // totally staked amount
    pub total_staked_amount: u64,
    // total staked card count
    pub total_stake_card: u64,
    
    // so this value would be 100 for 1% reward
    // pub tier_percent: [u16; 10],
    pub tier_grades: [u16; 10],
    pub available_tier: u8,

    // reserved space
    pub reserved: [u128; 4]
}
