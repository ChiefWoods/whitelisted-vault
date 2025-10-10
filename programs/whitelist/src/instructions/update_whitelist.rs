use anchor_lang::prelude::*;

use crate::{error::WhitelistedError, Config, Whitelist, CONFIG_SEED, WHITELIST_SEED};

#[derive(Accounts)]
pub struct UpdateWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [CONFIG_SEED],
        bump = config.bump,
        has_one = admin @ WhitelistedError::InvalidAdmin,
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [WHITELIST_SEED, whitelisted_address.key().as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    /// CHECK: Authority of token account
    pub whitelisted_address: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateWhitelist<'info> {
    pub fn handler(ctx: Context<UpdateWhitelist>, is_blocked: bool) -> Result<()> {
        let UpdateWhitelist { whitelist, .. } = ctx.accounts;

        whitelist.is_blocked = is_blocked;

        Ok(())
    }
}
