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
	pub struct ExecuteProposal<'info> {
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

		#[account(
			mut,
		)]
		/// CHECK: implement manual checks if needed
		pub source: UncheckedAccount<'info>,

		#[account(
			mut,
		)]
		/// CHECK: implement manual checks if needed
		pub destination: UncheckedAccount<'info>,

		#[account(
			owner=Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
		)]
		pub authority: Signer<'info>,

		#[account(
			mut,
			owner=Pubkey::from_str("11111111111111111111111111111111").unwrap(),
		)]
		pub funding: Signer<'info>,

		#[account(
			init,
			payer = funding,
			associated_token::mint = mint,
			associated_token::authority = wallet,
			associated_token::token_program = token_program,
		)]
		pub assoc_token_account: Account<'info, TokenAccount>,

		/// CHECK: implement manual checks if needed
		pub wallet: UncheckedAccount<'info>,

		pub mint: Account<'info, Mint>,

		pub system_program: Program<'info, System>,

		pub token_program: Program<'info, Token>,

		pub csl_spl_token_v0_0_0: Program<'info, Token>,

		pub csl_spl_assoc_token_v0_0_0: Program<'info, AssociatedToken>,
	}

	impl<'info> ExecuteProposal<'info> {
		pub fn cpi_csl_spl_token_transfer(&self, amount: u64) -> Result<()> {
			anchor_spl::token::transfer(
				CpiContext::new(self.csl_spl_token_v0_0_0.to_account_info(), 
					anchor_spl::token::Transfer {
						from: self.source.to_account_info(),
						to: self.destination.to_account_info(),
						authority: self.authority.to_account_info()
					}
				),
				amount, 
			)
		}
	}


/// Execute a proposal if the threshold is met
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[]` multisig: [MultisigConfig] The multisig account
/// 2. `[writable]` proposal: [Proposal] The proposal to execute
/// 3. `[writable]` source: [AccountInfo] The source account.
/// 4. `[writable]` destination: [AccountInfo] The destination account.
/// 5. `[signer]` authority: [AccountInfo] The source account's owner/delegate.
/// 6. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 7. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 8. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 9. `[]` mint: [Mint] The token mint for the new associated token account
/// 10. `[]` system_program: [AccountInfo] System program
/// 11. `[]` token_program: [AccountInfo] SPL Token program
/// 12. `[]` csl_spl_token_v0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
/// 13. `[]` csl_spl_assoc_token_v0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
///
/// Data:
/// - proposal_seed_proposal_id: [u64] Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
pub fn handler(
	ctx: Context<ExecuteProposal>,
) -> Result<()> {
    // Implement your business logic here...
	
	// Cpi calls wrappers
	ctx.accounts.cpi_csl_spl_token_transfer(
		Default::default(),
	)?;

	Ok(())
}
