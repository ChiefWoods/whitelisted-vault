use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut,
            PodStateWithExtensionsMut,
        },
        pod::PodAccount,
    },
    token_interface::{Mint, TokenAccount},
};

use crate::{error::WhitelistedError, Whitelist, EXTRA_ACCOUNT_METAS_SEED, WHITELIST_SEED};

#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::authority = owner,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList Account
    #[account(
        seeds = [EXTRA_ACCOUNT_METAS_SEED, mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        seeds = [WHITELIST_SEED, whitelisted_address.key().as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    /// CHECK: Authority of token account
    pub whitelisted_address: UncheckedAccount<'info>,
}

impl<'info> TransferHook<'info> {
    fn check_is_transferring(&mut self) -> Result<()> {
        let source_token_info = self.source_token.to_account_info();
        let mut account_data_ref = source_token_info.try_borrow_mut_data()?;

        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
        let account_extension = account.get_extension_mut::<TransferHookAccount>()?;

        require!(
            bool::from(account_extension.transferring),
            WhitelistedError::NotTransferring
        );

        Ok(())
    }

    pub fn handler(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
        ctx.accounts.check_is_transferring()?;

        require!(
            !ctx.accounts.whitelist.is_blocked,
            WhitelistedError::AddressBlocked
        );

        Ok(())
    }
}
