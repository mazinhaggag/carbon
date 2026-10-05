use super::super::types::*;

// Not in the on-chain IDL yet: create_config for a base mint with a transfer
// hook, the hook program after the quote mint. Layout read from mainnet
// (HOOKI's launch, Oct 5 2026).

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd825013958e21929")]
pub struct CreateConfigWithTransferHook {
    pub config_parameters: ConfigParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateConfigWithTransferHookInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
    pub leftover_receiver: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub transfer_hook_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateConfigWithTransferHook {
    type ArrangedAccounts = CreateConfigWithTransferHookInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let config = next_account(&mut iter)?;
        let fee_claimer = next_account(&mut iter)?;
        let leftover_receiver = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let transfer_hook_program = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(CreateConfigWithTransferHookInstructionAccounts {
            config,
            fee_claimer,
            leftover_receiver,
            quote_mint,
            transfer_hook_program,
            payer,
            system_program,
            event_authority,
            program,
        })
    }
}
