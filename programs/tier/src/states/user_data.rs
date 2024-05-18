use crate::constants::{ONE_DAY, REWARD_DENOMIATOR};
use crate::error::StakingError;
use crate::states::GlobalState;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserData {
    // staker
    pub user: Pubkey,
    // staked amount
    pub amount: u64,
    // last claimed time
    pub last_reward_time: u64,
    // staked time
    pub staked_time: i64,
    // data seed
    pub seed_key: Pubkey,

    // reserved space
    pub reserved: [u128; 2]
}
