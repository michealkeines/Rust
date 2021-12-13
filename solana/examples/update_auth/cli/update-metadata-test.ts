import {
  Keypair,
  Connection,
  SystemProgram,
  TransactionInstruction,
} from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';

import {
  MetadataCategory,sendTransactionWithRetry
} from 'oyster-common';
//import { sendTransactionWithRetry} from '../contexts/connection';
import {PublicKey,clusterApiUrl} from '@solana/web3.js';
  import  { updateMetadata, Data, Creator } from '../index';
  import {AccountLayout, Token, TOKEN_PROGRAM_ID} from '@solana/spl-token';

  //import {Numberu64} from '../dist';


  export async function update(): Promise<void> {
    console.log("Updating meta data");
    const updateInstructions: TransactionInstruction[] = [];
    const updateSigners: Keypair[] = [];
    // export class Creator {
    //   address: StringPublicKey;
    //   verified: boolean;
    //   share: number;

    let programId = new PublicKey(
      "6YX3wuJixYZ35xrN7wTbBiJVB8p4pPbVuCZjLUqhx2C3"
      );
      let payer = new PublicKey(
        "AtcZrTWaadnxytJRLAj878abq5gw8Ymrp2Ks7jP6q7ov"
        );
      let args = {address: "6YX3wuJixYZ35xrN7wTbBiJVB8p4pPbVuCZjLUqhx2C3",verified: true,share:1};
    const owner = new Creator(args);
    const metadata = {
      name: "test",
      symbol: 'attributes.symbol',
      creators: owner,
      description: "attributes.description",
      sellerFeeBasisPoints: 1,
      image: "attributes.image",
      animation_url: undefined,
      attributes: undefined,
      external_url: "attributes.external_url",
      properties: {
        files: [],
        category: MetadataCategory.Image}
      };

    // TODO: connect to testnet arweave
    const arweaveLink = `https://arweave.net/123`;
    await updateMetadata(
      new Data({
        name: "test",
        symbol: "test.sym",
        uri: arweaveLink,
        creators: [metadata.creators],
        sellerFeeBasisPoints: metadata.sellerFeeBasisPoints,
      }),
      undefined,
      undefined,
      "6YX3wuJixYZ35xrN7wTbBiJVB8p4pPbVuCZjLUqhx2C3",
      "AtcZrTWaadnxytJRLAj878abq5gw8Ymrp2Ks7jP6q7ov",
      updateInstructions,
      "6YX3wuJixYZ35xrN7wTbBiJVB8p4pPbVuCZjLUqhx2C3",
    );
    const connection = new Connection(clusterApiUrl('devnet'));
    const wallet = useWallet();
    const txid = await sendTransactionWithRetry(
      connection,
      wallet,
      updateInstructions,
      updateSigners,
    );
    console.log('Metadata updated', txid);

  }