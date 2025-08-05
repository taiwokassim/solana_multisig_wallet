pub mod common;

use std::str::FromStr;
use {
    common::{
		get_program_test,
		multisig_wallet_ix_interface,
		csl_spl_token_ix_interface,
		csl_spl_assoc_token_ix_interface,
	},
    solana_program_test::tokio,
    solana_sdk::{
        account::Account, pubkey::Pubkey, rent::Rent, signature::Keypair, signer::Signer, system_program,
    },
};


#[tokio::test]
async fn execute_proposal_ix_success() {
	let mut program_test = get_program_test();

	// PROGRAMS
	program_test.prefer_bpf(true);

	program_test.add_program(
		"account_compression",
		Pubkey::from_str("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK").unwrap(),
		None,
	);

	program_test.add_program(
		"noop",
		Pubkey::from_str("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV").unwrap(),
		None,
	);

	program_test.add_program(
		"csl_spl_token",
		Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
		None,
	);

	program_test.add_program(
		"csl_spl_assoc_token",
		Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
		None,
	);

	// DATA
	let proposal_seed_proposal_id: u64 = Default::default();

	// KEYPAIR
	let fee_payer_keypair = Keypair::new();
	let authority_keypair = Keypair::new();
	let funding_keypair = Keypair::new();
	let mint_keypair = Keypair::new();

	// PUBKEY
	let fee_payer_pubkey = fee_payer_keypair.pubkey();
	let authority_pubkey = authority_keypair.pubkey();
	let funding_pubkey = funding_keypair.pubkey();
	let source_pubkey = Pubkey::new_unique();
	let destination_pubkey = Pubkey::new_unique();
	let wallet_pubkey = Pubkey::new_unique();
	let mint_pubkey = mint_keypair.pubkey();
	let token_program_pubkey = csl_spl_token_ix_interface::ID;
	let system_program_pubkey = Pubkey::new_unique();

	// EXECUTABLE PUBKEY
	let csl_spl_token_v0_0_0_pubkey = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
	let csl_spl_assoc_token_v0_0_0_pubkey = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap();

	// PDA
	let (multisig_pda, _multisig_pda_bump) = Pubkey::find_program_address(
		&[
			b"multisig",
		],
		&multisig_wallet::ID,
	);

	let (proposal_pda, _proposal_pda_bump) = Pubkey::find_program_address(
		&[
			b"proposal",
			multisig_pubkey.as_ref(),
			proposal_seed_proposal_id.to_le_bytes().as_ref(),
		],
		&multisig_wallet::ID,
	);

	let (assoc_token_account_pda, _assoc_token_account_pda_bump) = Pubkey::find_program_address(
		&[
			wallet_pubkey.as_ref(),
			token_program_pubkey.as_ref(),
			mint_pubkey.as_ref(),
		],
		&csl_spl_token_ix_interface::ID,
	);

	// ACCOUNT PROGRAM TEST SETUP
	program_test.add_account(
		fee_payer_pubkey,
		Account {
			lamports: 1_000_000_000_000,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	program_test.add_account(
		authority_pubkey,
		Account {
			lamports: 0,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	program_test.add_account(
		funding_pubkey,
		Account {
			lamports: 1_000_000_000_000,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	// INSTRUCTIONS
	let (mut banks_client, _, recent_blockhash) = program_test.start().await;

	let ix = multisig_wallet_ix_interface::execute_proposal_ix_setup(
		&fee_payer_keypair,
		multisig_pda,
		proposal_pda,
		source_pubkey,
		destination_pubkey,
		&authority_keypair,
		&funding_keypair,
		assoc_token_account_pda,
		wallet_pubkey,
		mint_pubkey,
		system_program_pubkey,
		token_program_pubkey,
		csl_spl_token_v0_0_0_pubkey,
		csl_spl_assoc_token_v0_0_0_pubkey,
		proposal_seed_proposal_id,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());

}
