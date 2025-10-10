use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};

use crate::{Vault, VAULT_SEED};

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = Vault::DISCRIMINATOR.len() + Vault::INIT_SPACE,
        seeds = [VAULT_SEED],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(mint::token_program = token_program)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializeVault<'info> {
    pub fn handler(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.vault.set_inner(Vault {
            bump: ctx.bumps.vault,
        });

        Ok(())
    }
}
