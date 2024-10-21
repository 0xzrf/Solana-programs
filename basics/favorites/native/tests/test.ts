import { Buffer } from 'node:buffer';
import { describe, test } from 'node:test';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { start } from 'solana-bankrun';

describe('Account Data!', async () => {
  test('Increment counter for two different PDAs', async () => {
    const context = await start([], []);
    const { payer, banksClient } = context
    const program = await context.deployProgram('./native/program/target/deploy/program.so');

    // Create two different users
    const user1 = Keypair.generate();
    const user2 = Keypair.generate();

    // Function to derive PDA for a user
    const derivePDA = (user: Keypair) => {
      const [pda] = PublicKey.findProgramAddressSync(
        [Buffer.from('my_seed'), Buffer.from('another_seed'), user.publicKey.toBuffer()],
        program
      );
      return pda;
    };

    // Derive PDAs for both users
    const pda1 = derivePDA(user1);
    const pda2 = derivePDA(user2);

    // Function to create and send transaction
    const sendTransaction = async (user: Keypair, pda: PublicKey) => {
      const ix = new TransactionInstruction({
        programId: program,
        keys: [
          { pubkey: user.publicKey, isSigner: true, isWritable: true },
          { pubkey: pda, isSigner: false, isWritable: true },
        ],
        data: Buffer.from([]),
      });

      const tx = new Transaction().add(ix);
      const txHash = await context.sendTransactio(tx, [user]);
      console.log(`Transaction hash: ${txHash}`);
      
      // Get and print logs
      const txInfo = await context.getTransaction(txHash);
      console.log('Transaction logs:', txInfo?.meta?.logMessages);
    };

    // Increment counter for user1 twice
    await sendTransaction(user1, pda1);
    await sendTransaction(user1, pda1);

    // Increment counter for user2 once
    await sendTransaction(user2, pda2);

    // Increment counter for user1 one more time
    await sendTransaction(user1, pda1);

    // The logs should show that user1's counter is at 3, while user2's counter is at 1
  });
});
