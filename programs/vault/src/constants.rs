use anchor_lang::prelude::*;

#[constant]
pub const VAULT_SEED: &[u8] = b"vault";
pub const WHITELIST_PROGRAM_ID: Pubkey = crate::whitelist::ID;
