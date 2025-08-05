use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};




	#[derive(Accounts)]
	#[instruction(
		destination: Pubkey,
		amount: u64,
		token_mint: Pubkey,
		expiration_time: i64,
		proposal_seed_proposal_id: u64,
	)]
	pub struct CreateProposal<'info> {
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

		#[account(
			init,
			space=487,
			payer=fee_payer,
			seeds = [
				b"proposal",
				multisig.key().as_ref(),
				proposal_seed_proposal_id.to_le_bytes().as_ref(),
			],
			bump,
		)]
		pub proposal: Account<'info, Proposal>,

		pub creator: Signer<'info>,

		pub system_program: Program<'info, System>,
	}

/// Create a new transaction proposal
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable]` multisig: [MultisigConfig] The multisig account
/// 2. `[writable]` proposal: [Proposal] The proposal account to initialize
/// 3. `[signer]` creator: [AccountInfo] The account creating the proposal (must be a signer of the multisig)
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - destination: [Pubkey] The destination account for the transfer
/// - amount: [u64] The amount of tokens to transfer
/// - token_mint: [Pubkey] The mint of the token to transfer (can be null for SOL)
/// - expiration_time: [i64] Optional timestamp when the proposal expires (0 means no expiration)
/// - proposal_seed_proposal_id: [u64] Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
pub fn handler(
	ctx: Context<CreateProposal>,
	destination: Pubkey,
	amount: u64,
	token_mint: Pubkey,
	expiration_time: i64,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
