use super::pool_config::PoolConfig;

use carbon_core::{borsh, CarbonDeserialize};

// Not in the on-chain IDL yet: the config of a transfer-hook launch.
// PoolConfig's 1040 bytes of fields, then the hook program and 48 bytes of
// padding (HOOKI's config D4wYQUBh…, Oct 5 2026: every PoolConfig field at
// its usual offset, hook program 9eQyvzp3… at 1048).
#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x28dcc2fb29c77bfd")]
pub struct ConfigWithTransferHook {
    pub config: PoolConfig,
    pub transfer_hook_program: solana_pubkey::Pubkey,
    pub padding: [u64; 6],
}
