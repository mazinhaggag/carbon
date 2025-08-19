

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x14c6caedebf3b742")]
pub struct WithdrawLeftover{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct WithdrawLeftoverInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub virtual_pool: solana_pubkey::Pubkey,
    pub token_base_account: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub leftover_receiver: solana_pubkey::Pubkey,
    pub token_base_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawLeftover {
    type ArrangedAccounts = WithdrawLeftoverInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            pool_authority,
            config,
            virtual_pool,
            token_base_account,
            base_vault,
            base_mint,
            leftover_receiver,
            token_base_program,
            event_authority,
            program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(WithdrawLeftoverInstructionAccounts {
            pool_authority: pool_authority.pubkey,
            config: config.pubkey,
            virtual_pool: virtual_pool.pubkey,
            token_base_account: token_base_account.pubkey,
            base_vault: base_vault.pubkey,
            base_mint: base_mint.pubkey,
            leftover_receiver: leftover_receiver.pubkey,
            token_base_program: token_base_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}