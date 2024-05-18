use crate::{ constants::*, error::StakingError, states::* };
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use std::mem::size_of;

#[derive(Accounts)]
pub struct HelloInitialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init_if_needed,
        seeds = [GLOBAL_STATE_SEED],
        space = 8 + std::mem::size_of::<GlobalState>(),
        payer = authority,
        bump,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,
    // pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> HelloInitialize<'info> {
    pub fn validate(&self) -> Result<()> {
        if self.global_state.is_initialized == 1 {
            require!(
                self.global_state.authority.eq(&self.authority.key()),
                StakingError::NotAllowedAuthority
            )
        }
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handle(
    ctx: Context<HelloInitialize>,
    new_authority: Pubkey,
    available_tier: u8,
) -> Result<()> {
    msg!("self.global_state.is_initialized = {:1}", true);
    let accts = ctx.accounts;
    accts.global_state.is_initialized = 1;
    accts.global_state.authority = new_authority;
    accts.global_state.available_tier = available_tier;
    Ok(())
}