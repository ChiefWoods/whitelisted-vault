use anchor_lang::prelude::*;

use crate::{Whitelist, WHITELIST_SEED};

#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = Whitelist::DISCRIMINATOR.len() + Whitelist::INIT_SPACE,
        seeds = [WHITELIST_SEED, whitelisted_address.key().as_ref()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    /// CHECK: Authority of token account
    pub whitelisted_address: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeWhitelist<'info> {
    pub fn handler(ctx: Context<InitializeWhitelist>) -> Result<()> {
        let InitializeWhitelist {
            whitelist,
            whitelisted_address,
            ..
        } = ctx.accounts;

        whitelist.set_inner(Whitelist {
            address: whitelisted_address.key(),
            is_blocked: false,
            bump: ctx.bumps.whitelist,
        });

        Ok(())
    }
}
