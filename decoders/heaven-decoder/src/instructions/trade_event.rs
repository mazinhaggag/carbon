

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbddb7fd34ee661ee")]
pub struct TradeEvent{
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub total_creator_trading_fees: u64,
    pub total_fee_paid: u64,
}

pub struct TradeEventInstructionAccounts {
    pub liquidity_pool_state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub protocol_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TradeEvent {
    type ArrangedAccounts = TradeEventInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        if accounts.is_empty() { return None; }
        
        Some(TradeEventInstructionAccounts {
            liquidity_pool_state: accounts[0].pubkey,
            // User and protocol_config are not provided in the event
            user: solana_pubkey::Pubkey::default(),
            protocol_config: solana_pubkey::Pubkey::default(),
        })
    }
}
