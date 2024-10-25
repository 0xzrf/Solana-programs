import { AddedAccount, start } from "solana-bankrun";
import { PublicKey, Transaction, SystemProgram, TransactionInstruction } from "@solana/web3.js";


test("one transfer", async () => {
	const programId = PublicKey.unique();
	const context = await start([{name: "counter_solana_native", programId}], [])

	

	const payer = context.payer;
	const client = context.banksClient;

	const blockhash = context.lastBlockhash;

	const ixs = new TransactionInstruction({
		keys: [],
		programId
	})

	const ix = new Transaction()

	ix.recentBlockhash = blockhash;

	ix.add(ixs)

	ix.sign(payer)

	const simRes = await client.simulateTransaction(ix);
	const meta = await client.processTransaction(ix)

	console.log(simRes.meta?.logMessages)
	expect(true)
});

