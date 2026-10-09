use solana_pubkey::Pubkey;

/// The pool part of the buy that completes a curve (a "synthetic migration",
/// pump's Oct 2026 upgrade): after the `TradeEvent` for what the curve had
/// left and the `CompleteEvent`, the rest of the buy is priced on the
/// reserves the migration will open the PumpSwap pool with. The buyer's total
/// is the `TradeEvent` plus this.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize, PartialEq)]
pub struct PostCompleteBuyEventEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub quote_mint: Pubkey,
    pub timestamp: i64,
    pub base_out: u64,
    pub quote_in: u64,
    pub fee_basis_points: u64,
    pub fee: u64,
    pub creator_fee_basis_points: u64,
    pub creator_fee: u64,
    pub buyback_fee: u64,
    pub pool_base_reserves_before: u64,
    pub pool_quote_reserves_before: u64,
    pub pool_base_reserves_after: u64,
    pub pool_quote_reserves_after: u64,
}

impl PostCompleteBuyEventEvent {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 {
            return None;
        }
        if data[0..8] != [0x6f, 0xb0, 0x6d, 0x8b, 0x31, 0x6c, 0xd5, 0xfb] {
            return None;
        }
        let mut data_slice = &data[8..];
        borsh::BorshDeserialize::deserialize(&mut data_slice).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::cpi_event::CpiEvent;

    #[test]
    fn the_pool_part_of_a_completing_buy_decodes_as_an_event() {
        let buy = PostCompleteBuyEventEvent {
            user: Pubkey::new_unique(),
            mint: Pubkey::new_unique(),
            bonding_curve: Pubkey::new_unique(),
            quote_mint: Pubkey::new_unique(),
            timestamp: 1_791_500_000,
            base_out: 12_345_678_901,
            quote_in: 2_500_000_000,
            fee_basis_points: 95,
            fee: 23_750_000,
            creator_fee_basis_points: 30,
            creator_fee: 7_500_000,
            buyback_fee: 1_250_000,
            pool_base_reserves_before: 206_900_000_000_000,
            pool_quote_reserves_before: 84_990_359_037,
            pool_base_reserves_after: 206_887_654_321_099,
            pool_quote_reserves_after: 87_490_359_037,
        };
        // An anchor event CPI: the event tag, the event's discriminator, then
        // its fields.
        let mut data = vec![0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d];
        data.extend_from_slice(&[0x6f, 0xb0, 0x6d, 0x8b, 0x31, 0x6c, 0xd5, 0xfb]);
        data.extend_from_slice(&borsh::to_vec(&buy).unwrap());
        match CpiEvent::decode(&data) {
            Some(CpiEvent::PostCompleteBuyEvent(decoded)) => assert_eq!(decoded, buy),
            other => panic!("decoded as {:?}", other.is_some()),
        }
    }
}
