import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Mock } from "../target/types/mock";
import {Keypair} from "@solana/web3.js"
import wallet from "./private_key.json"

describe("mock", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Mock as Program<Mock>;
  const metadata = {
    name: "BLOB",
    symbol: "BLB",
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json"
  }

  it("Initializes the NFT", async () => {
    const payer = Keypair.fromSecretKey(new Uint8Array(wallet))
    console.log(provider.connection)
    const keypair = new Keypair()
    const txn = await program.methods.createNft(metadata.name, metadata.symbol, metadata.uri)
    .accounts({
      mintAccount: keypair.publicKey,
      payer: payer.publicKey
    })
    .signers([keypair, payer]) // Add payer to the signers array
    .rpc();

    console.log("Transaction sig:", txn)
  })

});
