use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};




	#[derive(Accounts)]
	#[instruction(
		proposal_seed_proposal_id: u64,
	)]
	pub struct ApproveProposal<'info> {
		#[account(
			mut,
		)]
		pub fee_payer: Signer<'info>,

		#[account(
			seeds = [
				b"multisig",
			],
			bump,
		)]
		pub multisig: Account<'info, MultisigConfig>,

		#[account(
			mut,
			seeds = [
				b"proposal",
				multisig.key().as_ref(),
				proposal_seed_proposal_id.to_le_bytes().as_ref(),
			],
			bump,
		)]
		pub proposal: Account<'info, Proposal>,

		pub signer: Signer<'info>,
	}

/// Approve a pending proposal
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[]` multisig: [MultisigConfig] The multisig account
/// 2. `[writable]` proposal: [Proposal] The proposal to approve
/// 3. `[signer]` signer: [AccountInfo] The signer approving the proposal (must be a signer of the multisig)
///
/// Data:
/// - proposal_seed_proposal_id: [u64] Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
pub fn handler(
	ctx: Context<ApproveProposal>,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
