"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const node_test_1 = require("node:test");
const web3_js_1 = require("@solana/web3.js");
const solana_bankrun_1 = require("solana-bankrun");
(0, node_test_1.describe)("Hello World tests", () => __awaiter(void 0, void 0, void 0, function* () {
    const PROGRAM_ID = web3_js_1.PublicKey.unique();
    const context = yield (0, solana_bankrun_1.start)([{ name: "hello_world", programId: PROGRAM_ID }], []); // Starts a solana validator similar to solana-test-validator, but much faster and swifter
    const client = context.banksClient;
    const payer = context.payer;
    (0, node_test_1.test)("Say hello", () => __awaiter(void 0, void 0, void 0, function* () {
        const ix = context.lastBlockhash;
        const txnInstruction = new web3_js_1.TransactionInstruction({
            keys: [{ isSigner: true, isWritable: true, pubkey: payer.publicKey }],
            programId: PROGRAM_ID,
            data: Buffer.alloc(0) // No data will be sent because the instruction does not accept any arguments
        });
        const tx = new web3_js_1.Transaction();
        tx.recentBlockhash = ix;
        tx.add(txnInstruction).sign(payer);
        const transaction = yield client.processTransaction(tx);
        transaction.logMessages.forEach((log) => {
            console.log(log);
        });
    }));
}));
