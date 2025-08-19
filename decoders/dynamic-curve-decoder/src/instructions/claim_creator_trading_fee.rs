

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x52dcfabd03556b2d")]
pub struct ClaimCreatorTradingFee{
    pub max_base_amount: u64,
    pub max_quote_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimCreatorTradingFeeInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub token_a_account: solana_pubkey::Pubkey,
    pub token_b_account: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub token_base_program: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimCreatorTradingFee {
    type ArrangedAccounts = ClaimCreatorTradingFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            pool_authority,
            pool,
            token_a_account,
            token_b_account,
            base_vault,
            quote_vault,
            base_mint,
            quote_mint,
            creator,
            token_base_program,
            token_quote_program,
            event_authority,
            program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ClaimCreatorTradingFeeInstructionAccounts {
            pool_authority: pool_authority.pubkey,
            pool: pool.pubkey,
            token_a_account: token_a_account.pubkey,
            token_b_account: token_b_account.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            creator: creator.pubkey,
            token_base_program: token_base_program.pubkey,
            token_quote_program: token_quote_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}