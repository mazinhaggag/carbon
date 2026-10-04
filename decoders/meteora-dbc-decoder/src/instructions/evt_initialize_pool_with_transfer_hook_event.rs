use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

// Emitted by the transfer-hook pool init; same fields as EvtInitializePool.

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dd589a435c14a0f6e")]
pub struct EvtInitializePoolWithTransferHookEvent {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub pool_type: u8,
    pub activation_point: u64,
}
