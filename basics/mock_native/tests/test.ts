import {ProgramTestContext, start} from "solana-bankrun"
import { describe, test } from "node:test"
import { assert } from "chai"
import {PublicKey, Transaction, TransactionInstruction} from "@solana/web3.js"


describe("testing the hello world test", async () => {
    const PROGRAM_ID = PublicKey.unique();
    const context = await start([{name: "mock_native", programId: PROGRAM_ID}], []);
    const payer = context.payer
    const client = context.banksClient

    test("test hello world logs", async () => {
        // console.log("Test started")
        const blockhash = context.lastBlockhash;

        const ix = new TransactionInstruction({
            keys: [{pubkey: payer.publicKey, isSigner: true, isWritable: true}],
            programId: PROGRAM_ID,
            data: Buffer.alloc(0)
        })

        const tx = new Transaction();
        tx.recentBlockhash = blockhash;
        tx.add(ix).sign(payer);

        const transaction = await client.processTransaction(tx);

        assert(transaction.logMessages[0].startsWith(`Program ${PROGRAM_ID}`))
        
    })
})