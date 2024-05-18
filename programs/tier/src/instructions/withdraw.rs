use crate::{constants::*, error::*, instructions::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
/// UserData Account will be closed when user withdraws tokens.
/// All lamports will go to super_authority wallet
/// In withdraw function, there is no claim part.
/// so Claim Instruction should be prior to Withdraw instruction
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub treasury: SystemAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        has_one = treasury
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
        mut,
        seeds = [USER_STAKING_DATA_SEED, user_data.seed_key.as_ref(), user.key().as_ref()],
        bump,
        has_one = user,
        close = treasury
    )]
    pub user_data: Box<Account<'info, UserData>>,    
    
    #[account(
        mut,
        seeds = [USER_STATE_SEED, user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = yoiu_mint,
        associated_token::authority = user
    )]
    pub user_yoiu_ata: Box<Account<'info, TokenAccount>>,

    #[account(address = global_state.yoiu_token_mint)]
    pub yoiu_mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Withdraw<'info> {
    fn withdraw_token_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.pool.to_account_info(),
                to: self.user_yoiu_ata.to_account_info(),
                authority: self.global_state.to_account_info(),
            },
        )
    }

    fn penalty_token_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.pool.to_account_info(),
                to: self.dao_treasury.to_account_info(),
                authority: self.global_state.to_account_info(),
            },
        )
    }

    fn validate(&self) -> Result<()> {
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp >= self.user_data.staked_time + 14 * 86400,
            WithdrawError::LockPeriodNotMet
        );
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<Withdraw>) -> Result<()> {
    let accts = ctx.accounts;
    let amount = accts.user_data.amount;
    let bump = ctx.bumps.get("global_state").unwrap();    

    // update user data
    // add totally staked amount
    accts.user_state.total_staked_amount = accts
        .user_state
        .total_staked_amount
        .checked_sub(amount)
        .unwrap();
    // increase totally staked card count
    accts.user_state.total_stake_card = accts
        .user_state
        .total_stake_card
        .checked_sub(1)
        .unwrap();

    // Update totally staked amount in global_state
    accts.global_state.total_staked_amount = accts
        .global_state
        .total_staked_amount
        .checked_sub(amount)
        .unwrap();

    // Update card count
    accts.global_state.total_stake_card = accts
        .global_state
        .total_stake_card
        .checked_sub(1)
        .unwrap();

    // global_state is owner of pool account, so it's seeds should be signer
    token::transfer(
        accts
            .withdraw_token_context()
            .with_signer(&[&[GLOBAL_STATE_SEED.as_ref(), &[*bump]]]),
        amount,
    )?;
    Ok(())
}

pub fn handle_emergency(ctx: Context<Withdraw>) -> Result<()> {
    let accts = ctx.accounts;
    let pre_amount = accts.user_data.amount;
    let bump = ctx.bumps.get("global_state").unwrap();

    msg!("handle_emergency: __1");

    // Calculate the fee (14%)
    let fee = pre_amount * 14 / 100;
    let _ = token::transfer(
        accts
            .penalty_token_context()
            .with_signer(&[&[GLOBAL_STATE_SEED.as_ref(), &[*bump]]]),
        fee,
    );
    let amount = pre_amount - fee;
    //let amount: u64 = accts.user_data.amount;
    msg!("handle_emergency: __2");

    // update user data
    // add totally staked amount
    accts.user_state.total_staked_amount = accts
        .user_state
        .total_staked_amount
        .checked_sub(amount)
        .unwrap();
    // increase totally staked card count
    accts.user_state.total_stake_card = accts
        .user_state
        .total_stake_card
        .checked_sub(1)
        .unwrap();

    // Update totally staked amount in global_state
    accts.global_state.total_staked_amount = accts
        .global_state
        .total_staked_amount
        .checked_sub(amount)
        .unwrap();

    // Update card count
    accts.global_state.total_stake_card = accts
        .global_state
        .total_stake_card
        .checked_sub(1)
        .unwrap();

    // global_state is owner of pool account, so it's seeds should be signer
    token::transfer(
        accts
            .withdraw_token_context()
            .with_signer(&[&[GLOBAL_STATE_SEED.as_ref(), &[*bump]]]),
        amount,
    )?;
    Ok(())
}
