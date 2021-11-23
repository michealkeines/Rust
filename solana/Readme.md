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

Program Derived Addresses (PDA):

We'd like some way for the program to own the X tokens while the intermediate program is open and waiting for other party's transaction.

but can programs be given user space owernship of a token account.

the trick is to assign token acount ownership to a PDA of the intermediate program.




