

use carbon_core::{borsh, CarbonDeserialize};

use crate::types::ConfigParameters;

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1da34a42bb77c31a90")]
pub struct EvtCreateConfigV2Event {
    pub config: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
    pub leftover_receiver: solana_pubkey::Pubkey,
    pub config_parameters: ConfigParameters,
}