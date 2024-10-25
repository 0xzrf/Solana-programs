import { describe, test } from 'mocha';
import { assert, expect } from 'chai';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { BanksClient, Rent, start , } from 'solana-bankrun';
import { BN } from 'bn.js';
// import { COUNTER_ACCOUNT_SIZE, PROGRAM_ID, createIncrementInstruction, deserializeCounterAccount } from '../ts';

describe('Counter Solana Native',  () => {
  console.log("describe started")

  let programId: PublicKey,  client:BanksClient, payer: Keypair, blockhash: string, rent: Rent; 

  beforeEach( async () => {
     programId = PublicKey.unique();
    const context = await start([{ name: 'counter_solana_native', programId }], []);
    // Randomly generate the program keypair and load the program to solana-bankrun
    client = context.banksClient;
    // Get the payer keypair from the context, this will be used to sign transactions with enough lamports
    payer = context.payer;
    blockhash = context.lastBlockhash;
    rent = await client.getRent();
  })

  // Get the rent object to calculate rent for the accounts
  test('check the logs', async () => {
    const counter = Keypair.generate();
    const createAccountInstruction: TransactionInstruction = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      lamports: Number(rent.minimumBalance(BigInt(8))),
      newAccountPubkey: counter.publicKey,
      programId,
      space: 8
    });
    

    const incrementInstruction: TransactionInstruction = new TransactionInstruction({
      programId: programId,
      keys: [
        {
          isSigner: true,
          isWritable: false,
          pubkey: counter.publicKey
        }
      ],
      data: new Buffer([0x0])
    }) 


    const tx = new Transaction().add(createAccountInstruction).add(incrementInstruction);


    tx.feePayer = payer.publicKey;

    tx.recentBlockhash = blockhash;
    tx.sign(payer, counter);
    await client.processTransaction(tx);

    const counterAccouuntInfo = await client.getAccount(counter.publicKey);

    assert(counterAccouuntInfo, "Account was not created")

    console.log(counterAccouuntInfo)
    console.log(counterAccouuntInfo.owner.toString(), programId.toString())

    const data = Buffer.from(counterAccouuntInfo.data)

    if (data.byteLength != 8) {
      new Error("The size ain't good enough")
    }
    const counterAccount = {count: new BN(data, 'le')} // The data variable of the account will contain the state variables

    assert(counterAccount.count.toNumber() == 1, "Count ain't even 1")

    console.log(counterAccount.count)

  });
})