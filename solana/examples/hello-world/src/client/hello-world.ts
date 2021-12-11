import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
  } from '@solana/web3.js';
  import fs from 'mz/fs';
  import path from 'path';
  import * as borsh from 'borsh';

  import { getPayer, getRpcUrl, createKeypairFromFile } from './utils';

  let connection: Connection;

  let payer: Keypair;

  let programId: PublicKey;

  let greetedPubkey: PublicKey;

  const program_path = path.resolve(__dirname, '../program/target/deploy');

  const so_path = path.join(program_path, 'solana_bpf_helloworld.so');

  const keypair_path = path.join(program_path, 'solana_bpf_helloworld-keypair.json');

  class GreetingAccount {
      counter = 0;
      constructor(fields: {counter: number} | undefined = undefined) {
        if (fields) {
            this.counter = fields.counter;
        }
      }
  }

  const GreetingSchem = new Map(
      [
          [GreetingAccount, {kind: 'struct', fields: [['counter','u32']]}]
      ]
  );

  const greeting_size = borsh.serialize(
      GreetingSchem, new GreetingAccount()
  ).length;

  export async function establishConnection(): Promise<void> {
      const rpcUrl = await getRpcUrl();
      console.log(rpcUrl);
      connection = new Connection(rpcUrl, 'confirmed');
      const version = await connection.getVersion();
      console.log('connection established', rpcUrl, version);
  }


export async function establishPayer(): Promise<void> {
    let fees  = 0;
    if (!payer) {
        const {feeCalculator} = await connection.getRecentBlockhash();
        fees += await connection.getMinimumBalanceForRentExemption(greeting_size);

        fees += feeCalculator.lamportsPerSignature * 100;

        payer = await getPayer();

    }

    let lamports = await connection.getBalance(payer.publicKey);

   if (lamports < fees) {
        const sig = await connection.requestAirdrop(
            payer.publicKey,
            5
        );
        const t = await connection.confirmTransaction(sig);
        console.log("sig conform ",t);
        lamports = await connection.getBalance(payer.publicKey);
   }

    console.log('Using Account', payer.publicKey.toBase58, 'containing', lamports/LAMPORTS_PER_SOL,'SOL to pay for fees');

}

export async function checkProgram(): Promise<void> {
    try {
        const programKeypair = await createKeypairFromFile(keypair_path);

        programId = programKeypair.publicKey;
        console.log(programId.toBase58()," this is program id");
    } catch(err) {
        const errmsg = (err as Error).message;
        throw new Error('failed to read keypair');
    }

    const programInfo = await connection.getAccountInfo(programId);

    if (programInfo == null) {
        if (fs.existsSync(so_path)) {
            throw new Error("program needs to be deployed");
        } else {
            throw new Error("program needs to be build and deployed");
        }
    } else if (!programInfo.executable) {
        throw new Error("program is not executable");
    }

    console.log(`using program ${programId.toBase58()}`);

    const seed = "kaines";
    greetedPubkey = await PublicKey.createWithSeed(
        payer.publicKey,
        seed,
        programId
    );

    const greetedAccount = await connection.getAccountInfo(greetedPubkey);
    if (greetedAccount === null) {
        console.log(`creating account`, greetedPubkey.toBase58(),'to say hello to');

        const lamports = await connection.getMinimumBalanceForRentExemption(greeting_size);

        const transaction = new Transaction().add(
            SystemProgram.createAccountWithSeed(
                {
                    fromPubkey: payer.publicKey,
                    basePubkey: payer.publicKey,
                    seed: seed,
                    newAccountPubkey: greetedPubkey,
                    lamports,
                    space: greeting_size,
                    programId
                }
            )   
        );
        await sendAndConfirmTransaction(connection, transaction, [payer]);
    }
}

export async function sayHello(): Promise<void> {
    console.log("saying hello to ", greetedPubkey.toBase58());
    const instruction = new TransactionInstruction({ // this is how we intialize a program input
        keys: [{pubkey: greetedPubkey, isSigner: false, isWritable: true}],
        programId,
        data: Buffer.alloc(0)
    });
    console.log(instruction);
    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer]
    );
}

export async function reportGreetings(): Promise<void> {
    const accountInfo = await connection.getAccountInfo(greetedPubkey);
    if (accountInfo === null) {
        throw new Error("cannot find the greeted account");
    }
    const greeting = borsh.deserialize(
        GreetingSchem, 
        GreetingAccount,
        accountInfo.data
    );
    console.log(greetedPubkey.toBase58(), 'has greeted ',greeting.counter,'times');
}
