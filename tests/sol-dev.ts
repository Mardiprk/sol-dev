import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolDev } from "../target/types/sol_dev";
import { assert } from "chai";

describe("sol-dev", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolDev as Program<SolDev>;

  it("Checks the wallet balance", async () => {
    const target = provider.wallet.publicKey;

    const balanceBefore = await provider.connection.getBalance(target);
    console.log("Balance before (SOL):", balanceBefore / anchor.web3.LAMPORTS_PER_SOL);

    const balanceFromProgram = await program.methods
      .checkBalance()
      .accounts({
        target,
      })
      .view(); 

    console.log("Balance (from smart contract):", balanceFromProgram / anchor.web3.LAMPORTS_PER_SOL);

    assert.equal(balanceFromProgram, balanceBefore);
  });
});