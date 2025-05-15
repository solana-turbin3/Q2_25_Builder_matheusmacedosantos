use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::StakeConfig;

// use crate::state::StakeConfig; /// idk if need this 

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        mut
    )]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + StakeConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, StakeConfig>,

     
    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = config,
        seeds = [b"rewards", config.key().as_ref()],
        bump
    )]
    pub rewards_mint: Account<'info, Mint>, 

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(
        &mut self, 
        points_per_Stake: u8,
        max_stake: u8,
        freeze_period: u32,
        rewards_bump: u8,
        bumps: &InitializeConfigBumps,
     ) -> Result<()> {

        self.config.set_inner(StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bump.rewards_mint,
            bumps: bumps.config,
        });

        Ok(())
    }
        

    }
