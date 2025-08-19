

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1407a9213a93a621")]
pub struct TransferPoolCreator{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferPoolCreatorInstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub new_creator: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferPoolCreator {
    type ArrangedAccounts = TransferPoolCreatorInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            virtual_pool,
            config,
            creator,
            new_creator,
            event_authority,
            program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(TransferPoolCreatorInstructionAccounts {
            virtual_pool: virtual_pool.pubkey,
            config: config.pubkey,
            creator: creator.pubkey,
            new_creator: new_creator.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}