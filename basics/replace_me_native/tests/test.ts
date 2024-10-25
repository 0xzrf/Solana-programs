import {ProgramTestContext, start} from "solana-bankrun"
import { describe, test } from "mocha"
import { assert } from "chai"
import {PublicKey, Transaction, TransactionInstruction, Keypair} from "@solana/web3.js"
// import * as borsh from "borsh"

describe("Tests the functionality of the solana native program", async () => {
    const context = start([{name: ""}])
})