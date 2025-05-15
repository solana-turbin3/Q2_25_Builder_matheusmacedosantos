use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub points_per_stake: u8,
    pub max_stake: u8,
    pub freeze_preiod: u32,
    pub rewards_bump: u8,
    pub bumps: u8,
}