use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x253a237ebe35e4c5")]
pub struct ClaimCashback {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimCashbackInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub user_volume_accumulator: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub user_volume_accumulator_wsol_token_account: solana_pubkey::Pubkey,
    pub user_wsol_token_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimCashback {
    type ArrangedAccounts = ClaimCashbackInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let user = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let user_volume_accumulator_wsol_token_account = next_account(&mut iter)?;
        let user_wsol_token_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(ClaimCashbackInstructionAccounts {
            user,
            user_volume_accumulator,
            quote_mint,
            quote_token_program,
            user_volume_accumulator_wsol_token_account,
            user_wsol_token_account,
            system_program,
            event_authority,
            program,
        })
    }
}
