use {
	multisig_wallet::{
			entry,
			ID as PROGRAM_ID,
	},
	solana_sdk::{
		entrypoint::{ProcessInstruction, ProgramResult},
		pubkey::Pubkey,
	},
	anchor_lang::prelude::AccountInfo,
	solana_program_test::*,
};

// Type alias for the entry function pointer used to convert the entry function into a ProcessInstruction function pointer.
pub type ProgramEntry = for<'info> fn(
	program_id: &Pubkey,
	accounts: &'info [AccountInfo<'info>],
	instruction_data: &[u8],
) -> ProgramResult;

// Macro to convert the entry function into a ProcessInstruction function pointer.
#[macro_export]
macro_rules! convert_entry {
	($entry:expr) => {
		// Use unsafe block to perform memory transmutation.
		unsafe { core::mem::transmute::<ProgramEntry, ProcessInstruction>($entry) }
	};
}

pub fn get_program_test() -> ProgramTest {
	let program_test = ProgramTest::new(
		"multisig_wallet",
		PROGRAM_ID,
		processor!(convert_entry!(entry)),
	);
	program_test
}
	
pub mod multisig_wallet_ix_interface {

	use {
		solana_sdk::{
			hash::Hash,
			signature::{Keypair, Signer},
			instruction::Instruction,
			pubkey::Pubkey,
			transaction::Transaction,
		},
		multisig_wallet::{
			ID as PROGRAM_ID,
			accounts as multisig_wallet_accounts,
			instruction as multisig_wallet_instruction,
		},
		anchor_lang::{
			prelude::*,
			InstructionData,
		}
	};

	pub fn create_multisig_ix_setup(
		fee_payer: &Keypair,
		multisig: Pubkey,
		system_program: Pubkey,
		signers: Vec<Pubkey>,
		threshold: u8,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = multisig_wallet_accounts::CreateMultisig {
			fee_payer: fee_payer.pubkey(),
			multisig: multisig,
			system_program: system_program,
		};

		let data = 	multisig_wallet_instruction::CreateMultisig {
				signers,
				threshold,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
		], recent_blockhash);

		return transaction;
	}

	pub fn create_proposal_ix_setup(
		fee_payer: &Keypair,
		multisig: Pubkey,
		proposal: Pubkey,
		creator: &Keypair,
		system_program: Pubkey,
		destination: Pubkey,
		amount: u64,
		token_mint: Pubkey,
		expiration_time: i64,
		proposal_seed_proposal_id: u64,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = multisig_wallet_accounts::CreateProposal {
			fee_payer: fee_payer.pubkey(),
			multisig: multisig,
			proposal: proposal,
			creator: creator.pubkey(),
			system_program: system_program,
		};

		let data = 	multisig_wallet_instruction::CreateProposal {
				destination,
				amount,
				token_mint,
				expiration_time,
				_proposal_seed_proposal_id: proposal_seed_proposal_id,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&creator,
		], recent_blockhash);

		return transaction;
	}

	pub fn approve_proposal_ix_setup(
		fee_payer: &Keypair,
		multisig: Pubkey,
		proposal: Pubkey,
		signer: &Keypair,
		proposal_seed_proposal_id: u64,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = multisig_wallet_accounts::ApproveProposal {
			fee_payer: fee_payer.pubkey(),
			multisig: multisig,
			proposal: proposal,
			signer: signer.pubkey(),
		};

		let data = 	multisig_wallet_instruction::ApproveProposal {
				_proposal_seed_proposal_id: proposal_seed_proposal_id,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&signer,
		], recent_blockhash);

		return transaction;
	}

	pub fn execute_proposal_ix_setup(
		fee_payer: &Keypair,
		multisig: Pubkey,
		proposal: Pubkey,
		source: Pubkey,
		destination: Pubkey,
		authority: &Keypair,
		funding: &Keypair,
		assoc_token_account: Pubkey,
		wallet: Pubkey,
		mint: Pubkey,
		system_program: Pubkey,
		token_program: Pubkey,
		csl_spl_token_v0_0_0: Pubkey,
		csl_spl_assoc_token_v0_0_0: Pubkey,
		proposal_seed_proposal_id: u64,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = multisig_wallet_accounts::ExecuteProposal {
			fee_payer: fee_payer.pubkey(),
			multisig: multisig,
			proposal: proposal,
			source: source,
			destination: destination,
			authority: authority.pubkey(),
			funding: funding.pubkey(),
			assoc_token_account: assoc_token_account,
			wallet: wallet,
			mint: mint,
			system_program: system_program,
			token_program: token_program,
			csl_spl_token_v0_0_0: csl_spl_token_v0_0_0,
			csl_spl_assoc_token_v0_0_0: csl_spl_assoc_token_v0_0_0,
		};

		let data = 	multisig_wallet_instruction::ExecuteProposal {
				_proposal_seed_proposal_id: proposal_seed_proposal_id,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&authority,
			&funding,
		], recent_blockhash);

		return transaction;
	}

	pub fn cancel_proposal_ix_setup(
		fee_payer: &Keypair,
		multisig: Pubkey,
		proposal: Pubkey,
		creator: &Keypair,
		proposal_seed_proposal_id: u64,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = multisig_wallet_accounts::CancelProposal {
			fee_payer: fee_payer.pubkey(),
			multisig: multisig,
			proposal: proposal,
			creator: creator.pubkey(),
		};

		let data = 	multisig_wallet_instruction::CancelProposal {
				_proposal_seed_proposal_id: proposal_seed_proposal_id,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&creator,
		], recent_blockhash);

		return transaction;
	}

	pub fn update_multisig_ix_setup(
		fee_payer: &Keypair,
		multisig: Pubkey,
		new_signers: Vec<Pubkey>,
		new_threshold: u8,
		all_current_signers_approved: bool,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = multisig_wallet_accounts::UpdateMultisig {
			fee_payer: fee_payer.pubkey(),
			multisig: multisig,
		};

		let data = 	multisig_wallet_instruction::UpdateMultisig {
				new_signers,
				new_threshold,
				all_current_signers_approved,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
		], recent_blockhash);

		return transaction;
	}

}

pub mod csl_spl_token_ix_interface {

	use {
		solana_sdk::{
			hash::Hash,
			signature::{Keypair, Signer},
			instruction::Instruction,
			pubkey::Pubkey,
			transaction::Transaction,
		},
		csl_spl_token::{
			ID as PROGRAM_ID,
			accounts as csl_spl_token_accounts,
			instruction as csl_spl_token_instruction,
		},
		anchor_lang::{
			prelude::*,
			InstructionData,
		}
	};

	declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

}

pub mod csl_spl_assoc_token_ix_interface {

	use {
		solana_sdk::{
			hash::Hash,
			signature::{Keypair, Signer},
			instruction::Instruction,
			pubkey::Pubkey,
			transaction::Transaction,
		},
		csl_spl_assoc_token::{
			ID as PROGRAM_ID,
			accounts as csl_spl_assoc_token_accounts,
			instruction as csl_spl_assoc_token_instruction,
		},
		anchor_lang::{
			prelude::*,
			InstructionData,
		}
	};

	declare_id!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

}
