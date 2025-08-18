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
pub enum AmmFeeOn {
    Quote = 0,
    Both = 1,
}