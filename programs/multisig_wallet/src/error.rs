// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.
//
// Docs: https://docs.codigo.ai/c%C3%B3digo-interface-description-language/specification#errors

use anchor_lang::prelude::*;

#[error_code]
pub enum MultisigWalletError {
	#[msg("The signer is not authorized for this multisig")]
	NotASigner,
	#[msg("The approval threshold has not been met")]
	ThresholdNotMet,
	#[msg("The proposal has expired")]
	ProposalExpired,
	#[msg("The proposal has already been executed")]
	ProposalAlreadyExecuted,
	#[msg("The proposal has been cancelled")]
	ProposalCancelled,
	#[msg("Not enough approvals to execute this proposal")]
	InsufficientApprovals,
	#[msg("All current signers must approve to update the multisig configuration")]
	NotAllSignersApproved,
	#[msg("Threshold must be greater than 0 and less than or equal to the number of signers")]
	InvalidThreshold,
	#[msg("Maximum number of signers exceeded")]
	MaxSignersExceeded,
}
