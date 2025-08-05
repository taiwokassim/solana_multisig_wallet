
use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
	pub id: u64,
	pub creator: Pubkey,
	pub destination: Pubkey,
	pub amount: u64,
	pub token_mint: Pubkey,
	pub approvals: Vec<Pubkey>,
	pub executed: bool,
	pub cancelled: bool,
	pub expiration_time: i64,
	pub multisig: Pubkey,
	pub nonce: u8,
}
