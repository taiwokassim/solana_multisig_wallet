use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};




	#[derive(Accounts)]
	#[instruction(
		new_signers: Vec<Pubkey>,
		new_threshold: u8,
		all_current_signers_approved: bool,
	)]
	pub struct UpdateMultisig<'info> {
		#[account(
			mut,
		)]
		pub fee_payer: Signer<'info>,

		#[account(
			mut,
			seeds = [
				b"multisig",
			],
			bump,
		)]
		pub multisig: Account<'info, MultisigConfig>,
	}

/// Update the signers or threshold of the multisig (requires approval from all current signers)
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable]` multisig: [MultisigConfig] The multisig account to update
///
/// Data:
/// - new_signers: [Vec<Pubkey>] The new set of authorized signers
/// - new_threshold: [u8] The new approval threshold
/// - all_current_signers_approved: [bool] Flag indicating that all current signers have approved this update
pub fn handler(
	ctx: Context<UpdateMultisig>,
	new_signers: Vec<Pubkey>,
	new_threshold: u8,
	all_current_signers_approved: bool,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
