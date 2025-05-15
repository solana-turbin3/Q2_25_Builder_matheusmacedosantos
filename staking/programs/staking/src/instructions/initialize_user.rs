use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        mut
    )]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = UserAccount::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,



    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

}

impl<'info> InitializeUser<'info> {
    pub fn initialize_config(
        &mut self, 
        bumps: &InitializeUserBumps,
     ) -> Result<()> {

        self.user_account.set_inner(UserAccount {
            points: 0,
            amount_staked: 0,
            bump: bumps.user_account,
        });
        Ok(())
    }
        

    }
 