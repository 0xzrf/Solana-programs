import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Mock } from "../target/types/mock";
import {Keypair} from "@solana/web3.js"

describe("mock", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Mock as Program<Mock>;
  const metadata = {
    name: "Zeref",
    symbol: "ZRF",
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json"
  }

  it("Initializes the NFT", async () => {
    const keypair = new Keypair()
    await program.methods.createNft(metadata.name, metadata.symbol, metadata.uri)
    .accounts({
      mintAccount: keypair.publicKey,
      payer: provider.wallet.publicKey
    })
    .signers([keypair])
    .rpc();

    console.log("Made a new NFT!!!")
  })

});
