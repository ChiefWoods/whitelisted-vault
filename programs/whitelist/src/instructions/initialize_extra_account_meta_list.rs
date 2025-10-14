use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::{EXTRA_ACCOUNT_METAS_SEED, WHITELIST_SEED};

#[derive(Accounts)]
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
            InitializeExtraAccountMetaList::extra_account_metas()?.len()
        ).unwrap()
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        Ok(vec![ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal {
                    bytes: WHITELIST_SEED.to_vec(),
                },
                Seed::AccountKey { index: 3 },
            ],
            false,
            false,
        )?])
    }

    pub fn handler(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        let extra_account_metas = InitializeExtraAccountMetaList::extra_account_metas()?;

        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anchor_lang::prelude::instruction::Instruction;
    use anchor_lang::{InstructionData, ToAccountMetas};
    use solana_keypair::Keypair;
    use solana_program::native_token::LAMPORTS_PER_SOL;
    use solana_signer::Signer;
    use spl_transfer_hook_interface::get_extra_account_metas_address;

    use crate::tests::constants::{PROGRAM_ID, SYSTEM_PROGRAM_ID, TOKEN_2022_PROGRAM_ID};
    use crate::tests::cpi::{
        InitializeConfigAccounts, InitializeConfigData, InitializeExtraAccountMetaListAccounts,
        InitializeExtraAccountMetaListData, InitializeMintAccounts, InitializeMintData,
        InitializeWhitelistAccounts, InitializeWhitelistData,
    };
    use crate::tests::pda::{get_config_pda, get_whitelist_pda};
    use crate::tests::utils::{build_and_send_transaction, init_wallet, setup};

    #[test]
    fn test_initialize_extra_account_meta_list() {
        let (litesvm, _default_payer) = &mut setup();

        let admin = init_wallet(litesvm, LAMPORTS_PER_SOL);
        let address1 = init_wallet(litesvm, LAMPORTS_PER_SOL);
        let mint = Keypair::new();

        let config_pda = get_config_pda();
        let whitelist_pda = get_whitelist_pda(&address1.pubkey());

        let ixs = vec![
            Instruction {
                accounts: InitializeConfigAccounts {
                    admin: admin.pubkey(),
                    config: config_pda,
                    system_program: SYSTEM_PROGRAM_ID,
                }
                .to_account_metas(None),
                data: InitializeConfigData {}.data(),
                program_id: PROGRAM_ID,
            },
            Instruction {
                accounts: InitializeWhitelistAccounts {
                    payer: admin.pubkey(),
                    whitelist: whitelist_pda,
                    whitelisted_address: address1.pubkey(),
                    system_program: SYSTEM_PROGRAM_ID,
                }
                .to_account_metas(None),
                data: InitializeWhitelistData {}.data(),
                program_id: PROGRAM_ID,
            },
            Instruction {
                accounts: InitializeMintAccounts {
                    mint: mint.pubkey(),
                    payer: admin.pubkey(),
                    system_program: SYSTEM_PROGRAM_ID,
                    token_program: TOKEN_2022_PROGRAM_ID,
                }
                .to_account_metas(None),
                data: InitializeMintData {}.data(),
                program_id: PROGRAM_ID,
            },
        ];

        let _ = build_and_send_transaction(litesvm, &[&admin, &mint], &admin.pubkey(), &ixs);

        let extra_account_meta_list_pda =
            get_extra_account_metas_address(&mint.pubkey(), &PROGRAM_ID);

        let ix = Instruction {
            accounts: InitializeExtraAccountMetaListAccounts {
                extra_account_meta_list: extra_account_meta_list_pda,
                mint: mint.pubkey(),
                payer: admin.pubkey(),
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None),
            data: InitializeExtraAccountMetaListData {}.data(),
            program_id: PROGRAM_ID,
        };

        let _ = build_and_send_transaction(litesvm, &[&admin], &admin.pubkey(), &[ix]);

        let extra_accounts_meta_list_acc =
            litesvm.get_account(&extra_account_meta_list_pda).unwrap();

        // 8 bytes for discriminator, 4 bytes for length, the rest is ExtraAccountMetaList
        let extra_account_meta_list_bytes = &extra_accounts_meta_list_acc.data[12..];
        // 4 bytes for count
        let (count_bytes, _extra_accounts_meta_bytes) = extra_account_meta_list_bytes.split_at(4);

        let count = u32::from_le_bytes(count_bytes.try_into().unwrap());

        assert_eq!(count, 1);
    }
}
