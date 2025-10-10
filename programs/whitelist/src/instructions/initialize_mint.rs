use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::Mint};

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 6,
        mint::authority = payer,
        extensions::transfer_hook::authority = payer,
        extensions::transfer_hook::program_id = crate::ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> InitializeMint<'info> {
    pub fn handler() -> Result<()> {
        Ok(())
    }
}
