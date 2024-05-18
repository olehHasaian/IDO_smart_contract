use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
//use spl_token_metadata::{state::Metadata, ID as MetadataProgramID};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        payer = user,
        space = 8 + core::mem::size_of::<GlobalState>()
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        mut,
        seeds = [POOL_SEED],
        bump
    )]
    pub pool: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [DAO_TREASURY_SEED],
        bump
    )]
    pub dao_treasury: Box<Account<'info, TokenAccount>>,
    
    #[account(
        init_if_needed,
        seeds = [USER_STAKING_DATA_SEED, data_seed.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + core::mem::size_of::<UserData>()
    )]
    pub user_data: Box<Account<'info, UserData>>,

    #[account(
        init_if_needed,
        seeds = [USER_STATE_SEED, user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + core::mem::size_of::<UserState>()
    )]
    pub user_state: Box<Account<'info, UserState>>,

    /// CHECK: This is a random keypair for generating user_data
    pub data_seed: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = yoiu_mint,
        associated_token::authority = user
    )]
    pub user_yoiu_ata: Box<Account<'info, TokenAccount>>,

    #[account(address = global_state.yoiu_token_mint)]
    pub yoiu_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Stake<'info> {
    fn stake_token_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_yoiu_ata.to_account_info(),
                to: self.pool.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    // validate minimum deposit amount from metadata account
    pub fn validate(&self, amount: u64) -> Result<()> {
        // check minimum amount
        let deposit_minum = DEPOSIT_MINIMUM_AMOUNT
            .checked_mul(10u64.checked_pow(self.yoiu_mint.decimals as u32).unwrap())
            .unwrap();
        require!(amount >= deposit_minum, StakingError::InsufficientAmount);
        // check if userState is inited to avoid re-initialization attack
        if self.user_state.is_initialized == 1 {
            require!(
                self.user_state.user.eq(&self.user.key()),
                StakingError::IncorrectUserState
            );
        }        
        Ok(())
    }
}

pub fn decide_tier(amount: u64) -> u8 {
    match amount {
        0..=249 => 5,
        250..=1_499 => 4,
        1_500..=7_499 => 3,
        7_500..=19_999 => 2,
        _ => 1
    }
}

#[access_control(ctx.accounts.validate(amount))]
pub fn handle(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;

    let accts = ctx.accounts;

    // Init staking information in user_data
    accts.user_data.user = accts.user.key();
    accts.user_data.amount = amount;
    accts.user_data.staked_time = timestamp;
    accts.user_data.last_reward_time = timestamp as u64;
    accts.user_data.seed_key = accts.data_seed.key();

    // update user_state
    accts.user_state.is_initialized = 1;
    accts.user_state.user = accts.user.key();
    // add totally staked amount
    accts.user_state.total_staked_amount = accts
        .user_state
        .total_staked_amount
        .checked_add(amount)
        .unwrap();
    // increase totally staked card count
    accts.user_state.total_stake_card = accts
        .user_state
        .total_stake_card
        .checked_add(1)
        .unwrap();
    
    accts.user_state.tier = decide_tier(accts.user_state.total_staked_amount);

    // global state
    // Update totally staked amount in global_state
    accts.global_state.total_staked_amount = accts
        .global_state
        .total_staked_amount
        .checked_add(amount)
        .unwrap();
    // Update totally staked card in global_state
    accts.global_state.total_stake_card = accts
        .global_state
        .total_stake_card
        .checked_add(1)
        .unwrap();

    // transfer stake amount to pool
    token::transfer(accts.stake_token_context(), amount)?;

    Ok(())
}
