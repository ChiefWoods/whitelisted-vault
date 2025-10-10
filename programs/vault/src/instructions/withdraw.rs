use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{transfer_checked, Token2022, TransferChecked},
    token_interface::{Mint, TokenAccount},
};

use crate::{Vault, VAULT_SEED};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    #[account(
        seeds = [VAULT_SEED],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = withdrawer,
        associated_token::mint = mint,
        associated_token::authority = withdrawer,
        associated_token::token_program = token_program
    )]
    pub withdrawer_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Withdraw<'info> {
    pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let Withdraw {
            mint,
            token_program,
            vault,
            vault_token_account,
            withdrawer_token_account,
            ..
        } = ctx.accounts;

        let signer_seeds: &[&[u8]] = &[VAULT_SEED, &[vault.bump]];

        transfer_checked(
            CpiContext::new(
                token_program.to_account_info(),
                TransferChecked {
                    authority: vault.to_account_info(),
                    from: vault_token_account.to_account_info(),
                    mint: mint.to_account_info(),
                    to: withdrawer_token_account.to_account_info(),
                },
            )
            .with_signer(&[signer_seeds]),
            amount,
            mint.decimals,
        )
    }
}
