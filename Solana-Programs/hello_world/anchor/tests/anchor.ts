import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Anchor } from "../target/types/anchor";

describe("anchor", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Anchor as Program<Anchor>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);

    const confirmTransaction = await provider.connection.getConfirmedTransaction(tx, "confirmed")
    // This logs the msg! macro messages sometimes, but sometimes it doesn't
    if(confirmTransaction?.meta?.logMessages) {
      console.log("Program Logs:")

      confirmTransaction.meta.logMessages.forEach((log) => {
        console.log(log)
      })
    }else {
      console.log("No logs, stranger")
    }

  });
});
