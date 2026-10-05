use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

use crate::types::{SwapParameters2, SwapResult2};

// Emitted by swaps on a transfer-hook pool; same fields as EvtSwap2.
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d863ba8785e3372e7")]
pub struct EvtSwap2WithTransferHookEvent {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub swap_parameters: SwapParameters2,
    pub swap_result: SwapResult2,
    pub quote_reserve_amount: u64,
    pub migration_threshold: u64,
    pub current_timestamp: u64,
}
