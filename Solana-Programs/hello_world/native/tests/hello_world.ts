import {describe, test} from "node:test"
import {PublicKey, Transaction, TransactionInstruction} from "@solana/web3.js"
import {assert} from "chai"
import {start} from "solana-bankrun"

describe("Hello World tests", async () => {

    const PROGRAM_ID = PublicKey.unique();
    const context = await start([{name: "hello_world", programId: PROGRAM_ID}], []) // Starts a solana validator similar to solana-test-validator, but much faster and swifter
    const client = context.banksClient;
    const payer = context.payer;

    test("Say hello",async () => {
        const ix = context.lastBlockhash;
        const txnInstruction = new TransactionInstruction({
            keys: [{isSigner: true, isWritable: true, pubkey: payer.publicKey}],
            programId: PROGRAM_ID,
            data: Buffer.alloc(0) // No data will be sent because the instruction does not accept any arguments
        })

        const tx = new Transaction();
        tx.recentBlockhash = ix;
        tx.add(txnInstruction).sign(payer)

        const transaction = await client.processTransaction(tx);
        assert(transaction.logMessages[0].startsWith(`Program ${PROGRAM_ID}`));
        assert(transaction.logMessages[1] === 'Program log: Hello, Solana!');
        assert(transaction.logMessages[2] === `Program log: Our program's Program ID: ${PROGRAM_ID}`);
        assert(transaction.logMessages[3].startsWith(`Program ${PROGRAM_ID} consumed`));
        assert(transaction.logMessages[4] === `Program ${PROGRAM_ID} success`);
        assert(transaction.logMessages.length === 5);
    })
})