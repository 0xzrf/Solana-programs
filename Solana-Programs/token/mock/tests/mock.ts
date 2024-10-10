import * as anchor from "@coral-xyz/anchor";
import {Keypair} from "@solana/web3.js";

import { Program } from "@coral-xyz/anchor";
import { Mock } from "../target/types/mock";

describe("mock", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Mock as Program<Mock>;
  const payer =  provider.wallet as anchor.Wallet;

  const metadata = {
    name: "Zeref",
    symbol: "ZRF",
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json"
  }

  it("Initializes the token", async () => {
    const mintAccount = new Keypair();

    const txn = await program.methods.createToken(9, metadata.name, metadata.symbol, metadata.uri)
    .accounts({
      payer: payer.publicKey,
      mintAccount: mintAccount.publicKey
    })
    .signers([mintAccount])
    .rpc();

    console.log(`Created an Token here: ${mintAccount}`)
  })
});
