use anchor_lang::prelude::*;

use crate::instructions::*;
mod instructions;
mod state;
declare_id!("8irjy7j8LBMXjmd4DJcL11AkhYfPv2XTFwyhrS99KUx9");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
        bump: u8,
    ) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, bump, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }
}
