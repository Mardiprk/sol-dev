import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolDev } from "../target/types/sol_dev";

describe("sol-dev", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.SolDev as Program<SolDev>;

  it("Says Hello", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your Transaction Sign: ", tx);
  });
});