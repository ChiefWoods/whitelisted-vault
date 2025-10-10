use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{transfer_checked, Token2022, TransferChecked},
    token_interface::{Mint, TokenAccount},
};

use crate::{Vault, VAULT_SEED};

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub depositor: Signer<'info>,
    #[account(
        seeds = [VAULT_SEED],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub depositor_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> Deposit<'info> {
    pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let Deposit {
            depositor,
            depositor_token_account,
            mint,
            token_program,
            vault_token_account,
            ..
        } = ctx.accounts;

        transfer_checked(
            CpiContext::new(
                token_program.to_account_info(),
                TransferChecked {
                    authority: depositor.to_account_info(),
                    from: depositor_token_account.to_account_info(),
                    mint: mint.to_account_info(),
                    to: vault_token_account.to_account_info(),
                },
            ),
            amount,
            mint.decimals,
        )
    }
}
