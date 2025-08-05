
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use std::str::FromStr;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9joFwLjsjUQjJiywVzTk9UoxaeNXccCmphpB6bJePzgB");

#[program]
pub mod multisig_wallet {
    use super::*;

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
	pub fn create_multisig(ctx: Context<CreateMultisig>, signers: Vec<Pubkey>, threshold: u8) -> Result<()> {
		create_multisig::handler(ctx, signers, threshold)
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
	pub fn create_proposal(ctx: Context<CreateProposal>, destination: Pubkey, amount: u64, token_mint: Pubkey, expiration_time: i64, _proposal_seed_proposal_id: u64) -> Result<()> {
		create_proposal::handler(ctx, destination, amount, token_mint, expiration_time)
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
	pub fn approve_proposal(ctx: Context<ApproveProposal>, _proposal_seed_proposal_id: u64) -> Result<()> {
		approve_proposal::handler(ctx, )
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
	pub fn execute_proposal(ctx: Context<ExecuteProposal>, _proposal_seed_proposal_id: u64) -> Result<()> {
		execute_proposal::handler(ctx, )
	}

/// Cancel a pending proposal
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[]` multisig: [MultisigConfig] The multisig account
/// 2. `[writable]` proposal: [Proposal] The proposal to cancel
/// 3. `[signer]` creator: [AccountInfo] The creator of the proposal
///
/// Data:
/// - proposal_seed_proposal_id: [u64] Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
	pub fn cancel_proposal(ctx: Context<CancelProposal>, _proposal_seed_proposal_id: u64) -> Result<()> {
		cancel_proposal::handler(ctx, )
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
	pub fn update_multisig(ctx: Context<UpdateMultisig>, new_signers: Vec<Pubkey>, new_threshold: u8, all_current_signers_approved: bool) -> Result<()> {
		update_multisig::handler(ctx, new_signers, new_threshold, all_current_signers_approved)
	}



}
