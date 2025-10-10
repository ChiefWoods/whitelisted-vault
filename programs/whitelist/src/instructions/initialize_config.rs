use anchor_lang::prelude::*;

use crate::{Config, CONFIG_SEED};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [CONFIG_SEED],
        bump,
        space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn handler(ctx: Context<InitializeConfig>) -> Result<()> {
        let InitializeConfig { admin, config, .. } = ctx.accounts;

        config.set_inner(Config {
            admin: admin.key(),
            bump: ctx.bumps.config,
        });

        Ok(())
    }
}
