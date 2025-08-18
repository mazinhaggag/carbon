use carbon_core::borsh;

#[derive(
    borsh::BorshSerialize,
    borsh::BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Hash,
)]
pub struct TransferFeeExtensionParams {
    pub transfer_fee_basis_points: u16,
    pub max_fee: u64,
}