
use anchor_lang::prelude::*;

#[account]
pub struct MultisigConfig {
	pub signers: Vec<Pubkey>,
	pub threshold: u8,
	pub proposal_count: u64,
	pub nonce: u8,
}
