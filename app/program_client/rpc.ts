import {
  AnchorProvider,
  BN,
  IdlAccounts,
  Program,
  web3,
} from "@coral-xyz/anchor";
import { MethodsBuilder } from "@coral-xyz/anchor/dist/cjs/program/namespace/methods";
import { MultisigWallet } from "../../target/types/multisig_wallet";
import idl from "../../target/idl/multisig_wallet.json";
import * as pda from "./pda";

import { CslSplToken } from "../../target/types/csl_spl_token";
import idlCslSplToken from "../../target/idl/csl_spl_token.json";

import { CslSplAssocToken } from "../../target/types/csl_spl_assoc_token";
import idlCslSplAssocToken from "../../target/idl/csl_spl_assoc_token.json";



let _program: Program<MultisigWallet>;
let _programCslSplToken: Program<CslSplToken>;
let _programCslSplAssocToken: Program<CslSplAssocToken>;


export const initializeClient = (
    programId: web3.PublicKey,
    anchorProvider = AnchorProvider.env(),
) => {
    _program = new Program<MultisigWallet>(
        idl as never,
        programId,
        anchorProvider,
    );

    _programCslSplToken = new Program<CslSplToken>(
        idlCslSplToken as never,
        new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        anchorProvider,
    );
    _programCslSplAssocToken = new Program<CslSplAssocToken>(
        idlCslSplAssocToken as never,
        new web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
        anchorProvider,
    );

};

export type CreateMultisigArgs = {
  feePayer: web3.PublicKey;
  signers: web3.PublicKey[];
  threshold: number;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Initialize a new multisig wallet with the specified signers and threshold
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` multisig: {@link MultisigConfig} The multisig account to initialize
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - signers: {@link PublicKey[]} The initial set of authorized signers
 * - threshold: {@link number} The minimum number of approvals needed to execute a transaction
 */
export const createMultisigBuilder = (
	args: CreateMultisigArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<MultisigWallet, never> => {
  const [multisigPubkey] = pda.deriveMultisigPDA(_program.programId);

  return _program
    .methods
    .createMultisig(
      args.signers,
      args.threshold,
    )
    .accountsStrict({
      feePayer: args.feePayer,
      multisig: multisigPubkey,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Initialize a new multisig wallet with the specified signers and threshold
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` multisig: {@link MultisigConfig} The multisig account to initialize
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - signers: {@link PublicKey[]} The initial set of authorized signers
 * - threshold: {@link number} The minimum number of approvals needed to execute a transaction
 */
export const createMultisig = (
	args: CreateMultisigArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    createMultisigBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Initialize a new multisig wallet with the specified signers and threshold
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` multisig: {@link MultisigConfig} The multisig account to initialize
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - signers: {@link PublicKey[]} The initial set of authorized signers
 * - threshold: {@link number} The minimum number of approvals needed to execute a transaction
 */
export const createMultisigSendAndConfirm = async (
  args: Omit<CreateMultisigArgs, "feePayer"> & {
    signers: {
      feePayer: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return createMultisigBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer])
    .rpc();
}

export type CreateProposalArgs = {
  feePayer: web3.PublicKey;
  creator: web3.PublicKey;
  destination: web3.PublicKey;
  amount: bigint;
  tokenMint: web3.PublicKey;
  expirationTime: bigint;
  proposalSeedProposalId: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Create a new transaction proposal
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal account to initialize
 * 3. `[signer]` creator: {@link PublicKey} The account creating the proposal (must be a signer of the multisig)
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - destination: {@link PublicKey} The destination account for the transfer
 * - amount: {@link BigInt} The amount of tokens to transfer
 * - token_mint: {@link PublicKey} The mint of the token to transfer (can be null for SOL)
 * - expiration_time: {@link BigInt} Optional timestamp when the proposal expires (0 means no expiration)
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const createProposalBuilder = (
	args: CreateProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<MultisigWallet, never> => {
  const [multisigPubkey] = pda.deriveMultisigPDA(_program.programId);
    const [proposalPubkey] = pda.deriveProposalPDA({
        multisig: args.multisig,
        proposalId: args.proposalSeedProposalId,
    }, _program.programId);

  return _program
    .methods
    .createProposal(
      args.destination,
      new BN(args.amount.toString()),
      args.tokenMint,
      new BN(args.expirationTime.toString()),
      new BN(args.proposalSeedProposalId.toString()),
    )
    .accountsStrict({
      feePayer: args.feePayer,
      multisig: multisigPubkey,
      proposal: proposalPubkey,
      creator: args.creator,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Create a new transaction proposal
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal account to initialize
 * 3. `[signer]` creator: {@link PublicKey} The account creating the proposal (must be a signer of the multisig)
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - destination: {@link PublicKey} The destination account for the transfer
 * - amount: {@link BigInt} The amount of tokens to transfer
 * - token_mint: {@link PublicKey} The mint of the token to transfer (can be null for SOL)
 * - expiration_time: {@link BigInt} Optional timestamp when the proposal expires (0 means no expiration)
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const createProposal = (
	args: CreateProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    createProposalBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Create a new transaction proposal
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal account to initialize
 * 3. `[signer]` creator: {@link PublicKey} The account creating the proposal (must be a signer of the multisig)
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - destination: {@link PublicKey} The destination account for the transfer
 * - amount: {@link BigInt} The amount of tokens to transfer
 * - token_mint: {@link PublicKey} The mint of the token to transfer (can be null for SOL)
 * - expiration_time: {@link BigInt} Optional timestamp when the proposal expires (0 means no expiration)
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const createProposalSendAndConfirm = async (
  args: Omit<CreateProposalArgs, "feePayer" | "creator"> & {
    signers: {
      feePayer: web3.Signer,
      creator: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return createProposalBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      creator: args.signers.creator.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.creator])
    .rpc();
}

export type ApproveProposalArgs = {
  feePayer: web3.PublicKey;
  signer: web3.PublicKey;
  proposalSeedProposalId: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Approve a pending proposal
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal to approve
 * 3. `[signer]` signer: {@link PublicKey} The signer approving the proposal (must be a signer of the multisig)
 *
 * Data:
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const approveProposalBuilder = (
	args: ApproveProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<MultisigWallet, never> => {
  const [multisigPubkey] = pda.deriveMultisigPDA(_program.programId);
    const [proposalPubkey] = pda.deriveProposalPDA({
        multisig: args.multisig,
        proposalId: args.proposalSeedProposalId,
    }, _program.programId);

  return _program
    .methods
    .approveProposal(
      new BN(args.proposalSeedProposalId.toString()),
    )
    .accountsStrict({
      feePayer: args.feePayer,
      multisig: multisigPubkey,
      proposal: proposalPubkey,
      signer: args.signer,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Approve a pending proposal
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal to approve
 * 3. `[signer]` signer: {@link PublicKey} The signer approving the proposal (must be a signer of the multisig)
 *
 * Data:
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const approveProposal = (
	args: ApproveProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    approveProposalBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Approve a pending proposal
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal to approve
 * 3. `[signer]` signer: {@link PublicKey} The signer approving the proposal (must be a signer of the multisig)
 *
 * Data:
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const approveProposalSendAndConfirm = async (
  args: Omit<ApproveProposalArgs, "feePayer" | "signer"> & {
    signers: {
      feePayer: web3.Signer,
      signer: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return approveProposalBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      signer: args.signers.signer.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.signer])
    .rpc();
}

export type ExecuteProposalArgs = {
  feePayer: web3.PublicKey;
  source: web3.PublicKey;
  destination: web3.PublicKey;
  authority: web3.PublicKey;
  funding: web3.PublicKey;
  wallet: web3.PublicKey;
  mint: web3.PublicKey;
  proposalSeedProposalId: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Execute a proposal if the threshold is met
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal to execute
 * 3. `[writable]` source: {@link PublicKey} The source account.
 * 4. `[writable]` destination: {@link PublicKey} The destination account.
 * 5. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 6. `[writable, signer]` funding: {@link PublicKey} Funding account (must be a system account)
 * 7. `[writable]` assoc_token_account: {@link PublicKey} Associated token account address to be created
 * 8. `[]` wallet: {@link PublicKey} Wallet address for the new associated token account
 * 9. `[]` mint: {@link Mint} The token mint for the new associated token account
 * 10. `[]` system_program: {@link PublicKey} System program
 * 11. `[]` token_program: {@link PublicKey} SPL Token program
 * 12. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 * 13. `[]` csl_spl_assoc_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplAssocTokenProgram v0.0.0
 *
 * Data:
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const executeProposalBuilder = (
	args: ExecuteProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<MultisigWallet, never> => {
  const [multisigPubkey] = pda.deriveMultisigPDA(_program.programId);
    const [proposalPubkey] = pda.deriveProposalPDA({
        multisig: args.multisig,
        proposalId: args.proposalSeedProposalId,
    }, _program.programId);
    const [assocTokenAccountPubkey] = pda.CslSplTokenPDAs.deriveAccountPDA({
        wallet: args.wallet,
        tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        mint: args.mint,
    }, new web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"));

  return _program
    .methods
    .executeProposal(
      new BN(args.proposalSeedProposalId.toString()),
    )
    .accountsStrict({
      feePayer: args.feePayer,
      multisig: multisigPubkey,
      proposal: proposalPubkey,
      source: args.source,
      destination: args.destination,
      authority: args.authority,
      funding: args.funding,
      assocTokenAccount: assocTokenAccountPubkey,
      wallet: args.wallet,
      mint: args.mint,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
      tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
      cslSplTokenV000: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
      cslSplAssocTokenV000: new web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Execute a proposal if the threshold is met
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal to execute
 * 3. `[writable]` source: {@link PublicKey} The source account.
 * 4. `[writable]` destination: {@link PublicKey} The destination account.
 * 5. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 6. `[writable, signer]` funding: {@link PublicKey} Funding account (must be a system account)
 * 7. `[writable]` assoc_token_account: {@link PublicKey} Associated token account address to be created
 * 8. `[]` wallet: {@link PublicKey} Wallet address for the new associated token account
 * 9. `[]` mint: {@link Mint} The token mint for the new associated token account
 * 10. `[]` system_program: {@link PublicKey} System program
 * 11. `[]` token_program: {@link PublicKey} SPL Token program
 * 12. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 * 13. `[]` csl_spl_assoc_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplAssocTokenProgram v0.0.0
 *
 * Data:
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const executeProposal = (
	args: ExecuteProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    executeProposalBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Execute a proposal if the threshold is met
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal to execute
 * 3. `[writable]` source: {@link PublicKey} The source account.
 * 4. `[writable]` destination: {@link PublicKey} The destination account.
 * 5. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 6. `[writable, signer]` funding: {@link PublicKey} Funding account (must be a system account)
 * 7. `[writable]` assoc_token_account: {@link PublicKey} Associated token account address to be created
 * 8. `[]` wallet: {@link PublicKey} Wallet address for the new associated token account
 * 9. `[]` mint: {@link Mint} The token mint for the new associated token account
 * 10. `[]` system_program: {@link PublicKey} System program
 * 11. `[]` token_program: {@link PublicKey} SPL Token program
 * 12. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 * 13. `[]` csl_spl_assoc_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplAssocTokenProgram v0.0.0
 *
 * Data:
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const executeProposalSendAndConfirm = async (
  args: Omit<ExecuteProposalArgs, "feePayer" | "authority" | "funding"> & {
    signers: {
      feePayer: web3.Signer,
      authority: web3.Signer,
      funding: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return executeProposalBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      authority: args.signers.authority.publicKey,
      funding: args.signers.funding.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.authority, args.signers.funding])
    .rpc();
}

export type CancelProposalArgs = {
  feePayer: web3.PublicKey;
  creator: web3.PublicKey;
  proposalSeedProposalId: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Cancel a pending proposal
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal to cancel
 * 3. `[signer]` creator: {@link PublicKey} The creator of the proposal
 *
 * Data:
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const cancelProposalBuilder = (
	args: CancelProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<MultisigWallet, never> => {
  const [multisigPubkey] = pda.deriveMultisigPDA(_program.programId);
    const [proposalPubkey] = pda.deriveProposalPDA({
        multisig: args.multisig,
        proposalId: args.proposalSeedProposalId,
    }, _program.programId);

  return _program
    .methods
    .cancelProposal(
      new BN(args.proposalSeedProposalId.toString()),
    )
    .accountsStrict({
      feePayer: args.feePayer,
      multisig: multisigPubkey,
      proposal: proposalPubkey,
      creator: args.creator,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Cancel a pending proposal
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal to cancel
 * 3. `[signer]` creator: {@link PublicKey} The creator of the proposal
 *
 * Data:
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const cancelProposal = (
	args: CancelProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    cancelProposalBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Cancel a pending proposal
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[]` multisig: {@link MultisigConfig} The multisig account
 * 2. `[writable]` proposal: {@link Proposal} The proposal to cancel
 * 3. `[signer]` creator: {@link PublicKey} The creator of the proposal
 *
 * Data:
 * - proposal_seed_proposal_id: {@link BigInt} Auto-generated, from the input "proposal" for the its seed definition "Proposal", sets the seed named "proposal_id"
 */
export const cancelProposalSendAndConfirm = async (
  args: Omit<CancelProposalArgs, "feePayer" | "creator"> & {
    signers: {
      feePayer: web3.Signer,
      creator: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return cancelProposalBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      creator: args.signers.creator.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.creator])
    .rpc();
}

export type UpdateMultisigArgs = {
  feePayer: web3.PublicKey;
  newSigners: web3.PublicKey[];
  newThreshold: number;
  allCurrentSignersApproved: boolean;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Update the signers or threshold of the multisig (requires approval from all current signers)
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` multisig: {@link MultisigConfig} The multisig account to update
 *
 * Data:
 * - new_signers: {@link PublicKey[]} The new set of authorized signers
 * - new_threshold: {@link number} The new approval threshold
 * - all_current_signers_approved: {@link boolean} Flag indicating that all current signers have approved this update
 */
export const updateMultisigBuilder = (
	args: UpdateMultisigArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<MultisigWallet, never> => {
  const [multisigPubkey] = pda.deriveMultisigPDA(_program.programId);

  return _program
    .methods
    .updateMultisig(
      args.newSigners,
      args.newThreshold,
      args.allCurrentSignersApproved,
    )
    .accountsStrict({
      feePayer: args.feePayer,
      multisig: multisigPubkey,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Update the signers or threshold of the multisig (requires approval from all current signers)
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` multisig: {@link MultisigConfig} The multisig account to update
 *
 * Data:
 * - new_signers: {@link PublicKey[]} The new set of authorized signers
 * - new_threshold: {@link number} The new approval threshold
 * - all_current_signers_approved: {@link boolean} Flag indicating that all current signers have approved this update
 */
export const updateMultisig = (
	args: UpdateMultisigArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    updateMultisigBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Update the signers or threshold of the multisig (requires approval from all current signers)
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` multisig: {@link MultisigConfig} The multisig account to update
 *
 * Data:
 * - new_signers: {@link PublicKey[]} The new set of authorized signers
 * - new_threshold: {@link number} The new approval threshold
 * - all_current_signers_approved: {@link boolean} Flag indicating that all current signers have approved this update
 */
export const updateMultisigSendAndConfirm = async (
  args: Omit<UpdateMultisigArgs, "feePayer"> & {
    signers: {
      feePayer: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return updateMultisigBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer])
    .rpc();
}

// Getters

export const getMultisigConfig = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<MultisigWallet>["multisigConfig"]> => _program.account.multisigConfig.fetch(publicKey, commitment);

export const getProposal = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<MultisigWallet>["proposal"]> => _program.account.proposal.fetch(publicKey, commitment);
export module CslSplTokenGetters {
    export const getMint = (
        publicKey: web3.PublicKey,
        commitment?: web3.Commitment
    ): Promise<IdlAccounts<CslSplToken>["mint"]> => _programCslSplToken.account.mint.fetch(publicKey, commitment);
    
    export const getAccount = (
        publicKey: web3.PublicKey,
        commitment?: web3.Commitment
    ): Promise<IdlAccounts<CslSplToken>["account"]> => _programCslSplToken.account.account.fetch(publicKey, commitment);
}

