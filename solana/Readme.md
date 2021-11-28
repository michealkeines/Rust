# Solana

On Solana, smart contracts are called programs.

What the hell is a blockchain?

to answer that we need to look into the background 

Let's Imagine Alice has an asset A and Bob has an asset B, they would like to trade their assets but neither wants to send their asset first. After all what if the other party does not hold up their end of trade and runs away with both assets

the tradional way to solve this problem is to introduce a thrid party C which both A and B trust(like banks). A or B can go first and send their asset to C. C then waits for the other paryt to send their asset and only then does C releases both assets.

the block chain replaces the trusted thrid party C with code on blockchain, specifically a smart contrac that verifiably acts the same way a trusted thrid party would.

A smart contract is superior to a trusted thrid party because of a number of reasons like

can you be sure that the trusted third party isn't colluding with the person on the other side of the trade? you can sure with a smart contract because you can look at the code before running it.


Entry point macro takes in a function name, as every request to the deployed program will be proxied through that function.


the deployed program is executed using BPF loader, that itself is a program, thus it could be updated, so different versions of BPF loaders have different entrypoints.

# Solana programs are stateless

if you want to store state, use accounts. 

programs themselves are stored in accounts which are marked executable

each account can hold data and SOL. 
each account has an owner

only the owner may debit the account and adjust its data, crediting may be done by anyone.

Account:

    Address: 256-bit public key
    Balance: current SOL count
    Allocated Date Size: Size in Bytes
    Assigined Program Id: Owner of this account
    Executable: Yes/No

Accounts can only be owned by programs not user of the account.


As every transaction is handled by a program on solana (the system program). 
programs owned by programs and accounts that are marked as executable are owned by the BPF loader.
the system program is owned by native loader and have special privileges such as allocating memory and marking accoutns as executable.

although the program owns all basic SOL accounts, it can only transfer SOL from an account when the transaction has been signed by the private key of the SOL account being debited.

in theory programs have full autonomy over the accounts they own. it is up tothe program's createor to limit this autonomy and up to the users of the programs to verify the program's creator has really done so.

All the accounts to be read or written must be passed into the entrypoint function.

this allows the runtime to parallelise transactions.

if the runtime knows all the accounts that will be written to and read by everyone at all times, it can run those transactions in paralled that do not touch the same accoutns or touch that accounts by only read and dont write.

if a transaction violated this constraint and reads or writes to an accoutn of whihc the runtime has not been notified, that tranaction will fail.

Program Structure:

    ├─ src
    │  ├─ lib.rs         -> registering modules
    │  ├─ entrypoint.rs  -> entrypoint to the program
    │  ├─ instruction.rs -> program API, (de)serializing instruction data
    │  ├─ processor.rs   -> program logic
    │  ├─ state.rs       -> program objects, (de)serializing state
    │  ├─ error.rs       -> program specific errors

The flow of program:

-> Someone calls the entrypoint.
-> the entrypoint forwards to the processor.
-> the processor asks instruction.rs to decode the instruction_data argument from the entrypoint funciton.
-> using the decoded data, the processor will now decide which processing function to use to process the request.
-> the processor may use state.rs to encode or decode the state of an account which has been passed into the entrypoint.


Verfiable Delay Function:

A function that takes a fixed amount of time to execute that produces a proof that it ran, which can be verified in less time than it took to produce.

Siganture Format:

each digital signature is in the binary format and consumes 64 bytes.

Transaction Format:

A transaction contains a compact-array of signatures, followed by a message. Each item in the signature array is a digital signature of the given message.

the solana runtime verifies that the number of signatures matched the number in the first 8 bits of the message header.

it also verfies that each signature was signed by the private key corresponding to the public key at the same index in the message's account addresses array.

Message Header Format:

the message header contains three unsigned 8-bit values,

the first value is the number of required signatures in the containing transaction.
the second value is the number of the corresponding account addresses that are read-only.
the third value is the number of read-only accounts addresses not requiring signatures.

Instruction Format:

An instruction contains a program id index, followed by a compact-array of account address indexes, followed by a compact array of opaque 8bit data, the program id index is used to indentify an on-chain program that can interpret the opaque data. the program id index is a unique 8bit index to an account adddress in the message's array of account addresses.

Compact-Array Format:

A compact-array is serialized as the array length, followed by each array item. the array length is a special multi byte encoding called compact-u16

a compact-u16 is a multi-byte encoding of 16 bits. the first byte contains the lower 7 bits of the value in its lower 7bits, if the value is above 0x7f, the high bit is set and the next 7 bits of the value are placed into the lower 7bits of a second byte.

Account address is 32 bytes of arbitrary data.

Storing state between transactions:

if the program needs to store state between transactions, it does so using accounts. accounts inclusde metadata that tells the runtime who is allowed to access the data and how.

the lifetime of an accounts is expressed by a number of fractional native tokens called lamports. Accounts are held in validator memory and pay rent to stay there.

each validator periodically scans all accounts and collects rent.

any account that drops to zero lamports is purged, accounts can be able marked as rent-exempt, if they contain a sufficient number of lamports.

the 256-bit public key is used to lookup an account on-chain

read-only accounts:

transactions can indicate that some of the accounts it references be treated as readonly in order to enable paralled account processing, if a program attempts to modify a readonly account, the transaction is rejected by the runtime.

Program Derived Addresses (PDA):

We'd like some way for the program to own the X tokens while the intermediate program is open and waiting for other party's transaction.

but can programs be given user space owernship of a token account.

the trick is to assign token acount ownership to a PDA of the intermediate program.


Using a program derived address, a program may be given the authority over an account and later transfer that authority to another.

this is possible because the program can act as the signed in the transaction that gives authority.

For example, if two users want to make a wager on the outcome of a game in solana, they must each transfer their wager's assets to some intermediary that will honor their agreement. currently there is no way to implement this intermediary as a program in solana because the intermediary program cannot transfer the assets to the winner.

this capability is necessary for many Defi applications since they require assets to be transferred to an escrow agent untill some event occurs that determines the new owner.







