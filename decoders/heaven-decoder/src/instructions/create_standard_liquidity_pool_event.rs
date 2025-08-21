

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbd3883904b3ff994")]
pub struct CreateStandardLiquidityPoolEvent{
    pub pool_id: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub config_version: u16,
    pub initial_token_reserve: u64,
    pub initial_virtual_wsol_reserve: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateStandardLiquidityPoolEventInstructionAccounts {
    pub liquidity_pool_state: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub protocol_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateStandardLiquidityPoolEvent {
    type ArrangedAccounts = CreateStandardLiquidityPoolEventInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            liquidity_pool_state,
            payer,
            creator,
            mint,
            protocol_config,
            _remaining @ ..
        ] = accounts else { return None; };

        Some(CreateStandardLiquidityPoolEventInstructionAccounts {
            liquidity_pool_state: liquidity_pool_state.pubkey,
            payer: payer.pubkey,
            creator: creator.pubkey,
            mint: mint.pubkey,
            protocol_config: protocol_config.pubkey,
        })
    }
}
