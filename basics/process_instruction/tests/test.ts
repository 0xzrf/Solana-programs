import { describe, test } from 'node:test';
import { Buffer } from 'node:buffer';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { BanksClient, Rent, start, } from 'solana-bankrun';
import { BN } from 'bn.js';
import * as borsh from "borsh"

// import { COUNTER_ACCOUNT_SIZE, PROGRAM_ID, createIncrementInstruction, deserializeCounterAccount } from '../ts';



describe('Counter Solana Native', () => {
  console.log("describe started")


  class UserData {
    name: string;
    height: number;

    constructor(properties: { name: string; height: number }) {
      // super(properties);
      this.name = properties.name;
      this.height = properties.height;
    }

    toBuffer() {
      return Buffer.from(borsh.serialize(UserDataSchema, this))
    }
  } 

  const UserDataSchema = {"struct": {
    "name": "string",
    'height': "u32"
  }}

  beforeEach(async () => {
    
  })
  
  test("This is a test", async () => {
    const programId = PublicKey.unique();
    const context = await start([{ name: 'something', programId }], []);
    // Randomly generate the program keypair and load the program to solana-bankrun
    const client = context.banksClient;
    // Get the payer keypair from the context, this will be used to sign transactions with enough lamports
    const payer = context.payer;
    const blockhash = context.lastBlockhash;
    const rent = await client.getRent();
    console.log("test started?")
    const Jack = new UserData({
      name: "Jack",
      height: 9
    })
    const Griff = new UserData({
      name: "Griff",
      height: 9
    })

    const tx_instruction1: TransactionInstruction = new TransactionInstruction({
      keys: [{ pubkey: payer.publicKey, isSigner: true, isWritable: true }],
      programId,
      data: Jack.toBuffer()
    })

    const tx_instruction2: TransactionInstruction = new TransactionInstruction({
      ...tx_instruction1,
      data: Griff.toBuffer()
    })
    const txn = new Transaction().add(tx_instruction1).add(tx_instruction2);

    txn.recentBlockhash = blockhash;
    txn.feePayer = payer.publicKey;
    txn.sign(payer)
    const simtxn = await client.simulateTransaction(txn)
    await client.processTransaction(txn);

    console.log(simtxn.meta?.logMessages)

  })


})