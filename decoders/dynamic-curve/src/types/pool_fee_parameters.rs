
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct PoolFeeParameters {
    pub base_fee: BaseFeeParameters,
    pub dynamic_fee: Option<DynamicFeeParameters>,
}
