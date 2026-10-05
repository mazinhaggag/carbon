use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

// Not in the on-chain IDL yet: the pool account of a transfer-hook launch.
// Same 424-byte layout as VirtualPool under its own name (HOOKI's pool,
// Oct 5 2026: config, creator, mint and vaults at VirtualPool's offsets).
#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xeddbb8172abda923")]
pub struct TransferHookPool {
    pub volatility_tracker: VolatilityTracker,
    pub config: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub protocol_base_fee: u64,
    pub protocol_quote_fee: u64,
    pub partner_base_fee: u64,
    pub partner_quote_fee: u64,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub pool_type: u8,
    pub is_migrated: u8,
    pub is_partner_withdraw_surplus: u8,
    pub is_protocol_withdraw_surplus: u8,
    pub migration_progress: u8,
    pub is_withdraw_leftover: u8,
    pub is_creator_withdraw_surplus: u8,
    pub migration_fee_withdraw_status: u8,
    pub metrics: PoolMetrics,
    pub finish_curve_timestamp: u64,
    pub creator_base_fee: u64,
    pub creator_quote_fee: u64,
    pub padding_1: [u64; 7],
}
