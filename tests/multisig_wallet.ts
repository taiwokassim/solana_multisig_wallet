
import { AnchorProvider, BN, setProvider, web3 } from "@coral-xyz/anchor";
import * as multisigWalletClient from "../app/program_client";
import chai from "chai";
import { assert, expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
chai.use(chaiAsPromised);

const programId = new web3.PublicKey("9joFwLjsjUQjJiywVzTk9UoxaeNXccCmphpB6bJePzgB");

describe("multisig_wallet tests", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const systemWallet = (provider.wallet as NodeWallet).payer;

  it("First test", async () => {
    // Add your test here
  });
});
