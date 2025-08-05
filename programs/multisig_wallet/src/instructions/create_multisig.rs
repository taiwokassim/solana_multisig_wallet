use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};




	#[derive(Accounts)]
	#[instruction(
		signers: Vec<Pubkey>,
		threshold: u8,
	)]
	pub struct CreateMultisig<'info> {
		#[account(
			mut,
		)]
		pub fee_payer: Signer<'info>,

		#[account(
			init,
			space=342,
			payer=fee_payer,
			seeds = [
				b"multisig",
			],
			bump,
		)]
		pub multisig: Account<'info, MultisigConfig>,

		pub system_program: Program<'info, System>,
	}

/// Initialize a new multisig wallet with the specified signers and threshold
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable]` multisig: [MultisigConfig] The multisig account to initialize
/// 2. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - signers: [Vec<Pubkey>] The initial set of authorized signers
/// - threshold: [u8] The minimum number of approvals needed to execute a transaction
pub fn handler(
	ctx: Context<CreateMultisig>,
	signers: Vec<Pubkey>,
	threshold: u8,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
