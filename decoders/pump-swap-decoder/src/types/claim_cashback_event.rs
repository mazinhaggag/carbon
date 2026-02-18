use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ClaimCashbackEvent {
    pub user: solana_pubkey::Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub total_claimed: u64,
    pub total_cashback_earned: u64,
}
