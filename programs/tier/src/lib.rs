use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("8FcfefZgB9dcWPcAo6n3tx494SfjZ2dd4gPyv25w2E3d");

#[program]
pub mod tier {
    use super::*;

    pub fn initialize(
        _ctx: Context<Initialize>,
        new_authority: Pubkey,
        treasury: Pubkey,
        tier_grades: [u16; 10],
        available_tier: u8
    ) -> Result<()> {
        initialize::handle(
            _ctx,
            new_authority,
            treasury,
            tier_grades,
            available_tier,
        )
    }

    pub fn stake(
        _ctx: Context<Stake>, 
        amount: u64
    ) -> Result<()> {
        stake::handle(_ctx, amount)
    }

    pub fn withdraw(
        _ctx: Context<Withdraw>
    ) -> Result<()> {
        withdraw::handle(_ctx)
    }

    pub fn emergency_withdraw(
        _ctx: Context<Withdraw>
    ) -> Result<()> {
        msg!("emergency_withdraw is called");
        withdraw::handle_emergency(_ctx)
    }

    pub fn hello_world(
        _ctx: Context<HelloInitialize>,
        new_authority: Pubkey,
        available_tier: u8
    ) -> Result<()> {        
        hello_initialize::handle(_ctx, new_authority, available_tier)
    }

}

