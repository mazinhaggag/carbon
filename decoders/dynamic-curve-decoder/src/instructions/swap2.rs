
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x414b3f4ceb5b5b88")]
pub struct Swap2{
    pub params: SwapParameters2,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct Swap2InstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub input_token_account: solana_pubkey::Pubkey,
    pub output_token_account: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_base_program: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub referral_token_account: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Swap2 {
    type ArrangedAccounts = Swap2InstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            pool_authority,
            config,
            pool,
            input_token_account,
            output_token_account,
            base_vault,
            quote_vault,
            base_mint,
            quote_mint,
            payer,
            token_base_program,
            token_quote_program,
            referral_token_account,
            event_authority,
            program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(Swap2InstructionAccounts {
            pool_authority: pool_authority.pubkey,
            config: config.pubkey,
            pool: pool.pubkey,
            input_token_account: input_token_account.pubkey,
            output_token_account: output_token_account.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            payer: payer.pubkey,
            token_base_program: token_base_program.pubkey,
            token_quote_program: token_quote_program.pubkey,
            referral_token_account: referral_token_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}