use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::{Whitelist, EXTRA_ACCOUNT_METAS_SEED, WHITELIST_SEED};

#[derive(Accounts)]
#[instruction(whitelisted_address: Pubkey)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        payer = payer,
        seeds = [EXTRA_ACCOUNT_METAS_SEED, mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(
            InitializeExtraAccountMetaList::extra_account_metas(whitelist.key(), whitelisted_address)?.len()
        ).unwrap()
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [WHITELIST_SEED, whitelisted_address.as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    fn extra_account_metas(
        whitelist: Pubkey,
        whitelisted_address: Pubkey,
    ) -> Result<Vec<ExtraAccountMeta>> {
        Ok(vec![
            ExtraAccountMeta::new_with_pubkey(&whitelist, false, false)?,
            ExtraAccountMeta::new_with_pubkey(&whitelisted_address, false, false)?,
        ])
    }

    pub fn handler(
        ctx: Context<InitializeExtraAccountMetaList>,
        whitelisted_address: Pubkey,
    ) -> Result<()> {
        let extra_account_metas = InitializeExtraAccountMetaList::extra_account_metas(
            ctx.accounts.whitelist.key(),
            whitelisted_address,
        )?;

        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas,
        )?;

        Ok(())
    }
}
