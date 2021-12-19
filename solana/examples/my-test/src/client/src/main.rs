use program_idk::instruction::{ MetadataInstruction, CreateMetadataAccountArgs, UpdateMetadataAccountArgs, CreateMasterEditionArgs,MintNewEditionFromMasterEditionViaTokenArgs };
use program_idk::state::{ Data, Creator, Metadata, MasterEditionV2, Edition };

//use program_idk::utils::try_from_slice_unchecked;

use std::str::FromStr;
use borsh::{BorshDeserialize, BorshSerialize};

use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    instruction,
    signature::{Keypair, Signer},
    message::Message,
    transaction::Transaction
};
use {
    borsh::{
        maybestd::io::{Error, Write},
        schema::{BorshSchema, Declaration, Definition, Fields},
    },
    std::collections::HashMap,
};
use solana_program::{pubkey, system_instruction, sysvar};
use solana_client::rpc_client::RpcClient;
use solana_client_helpers::{Client, ClientResult, SplToken};
use std::{process::exit, sync::Arc};

use solana_sdk::{
    signature::{read_keypair_file},
    system_instruction::create_account,
};
use spl_token::{
    instruction::{initialize_account, initialize_mint, mint_to},
    state::{Account, Mint},
};

use program_idk::state::{
    EDITION, PREFIX
};

pub fn try_from_slice_unchecked<T: BorshDeserialize>(data: &[u8]) -> Result<T, Error> {
    let mut data_mut = data;
    let result = T::deserialize(&mut data_mut)?;
    Ok(result)
}

const TOKEN_PROGRAM_PUBKEY: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

fn establish_connection() -> Client{
    let arr = [
        247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
        195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
        129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
         93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
        144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
        210, 208, 188, 161,  37, 158,   6, 108, 252
      ];
    let payer = Keypair::from_bytes(&arr).unwrap(); // keypair for the deployed program
    
    let client = RpcClient::new_with_commitment("https://api.devnet.solana.com".into(), CommitmentConfig::confirmed());
  // let client = RpcClient::new_with_commitment("http://localhost:8899".into(), CommitmentConfig::confirmed());
    let client = Client { client, payer }; // Initialize a RpcClient


   // println!("Connection established {:?}\n",&client.get_version());
    client
}

fn create_metadata_account_test(req: &Client, program_id: &Pubkey) -> (Metadata, Pubkey, Pubkey) {
    let program_info = req.get_account(program_id).unwrap(); // get information about the programid

    if !program_info.executable { // check if it is executable
        println!("Not Executable.");
    }

    //println!("{:?}", program_info );

    let arr: [u8;64] = [
    247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
    195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
    129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
     93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
    144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
    210, 208, 188, 161,  37, 158,   6, 108, 252
  ];
    let payer: Keypair = Keypair::from_bytes(&arr).unwrap(); // 

    /// Metaplex token program id, we will use this to create a mint token
   

    let wall_arr = [244,194,241,225,15,191,246,159,209,33,185,63,66,83,136,245,229,137,231,131,162,22,138,183,202,60,4,145,145,154,181,47,49,225,231,18,254,186,11,157,121,238,174,50,155,59,5,159,240,115,170,215,77,184,187,3,206,62,248,3,130,161,22,169];

    let wallet = Keypair::from_bytes(&wall_arr).unwrap(); // this key pair will be the updated authroity in next process


    let program_key = program_id.clone(); // deployed program id
    let token_key = Pubkey::from_str(TOKEN_PROGRAM_PUBKEY).unwrap();
    let name = String::from("test 1"); // name in metadata
    let symbol = String::from("test 2"); // symbol in metadata
    let uri = String::from("https://arweave.net/WcQTJhsilu0gMOVCHxI5Q3WqGkyLhnb6grmVCE7NLH4"); // uri in metadata
    let create_new_mint = true;
    let mutable = true; // set if the metadata account should be mutable
    let new_mint = Keypair::new(); // create new keypair for mint
    let mint_key = new_mint.pubkey(); // set the created new pub key as mint_key


    // Metadata accounts are simply PDA addresses with derived key of `['metaplex', metaplex_program_id, mint_id]`.
    let metadata_seeds = &[PREFIX.as_bytes(), &program_key.as_ref(), mint_key.as_ref()];

    // pubkey derived using seed
    let (metadata_key, _) = Pubkey::find_program_address(metadata_seeds, &program_key);
    //println!("all set");

    // Initialize insturctions to create a new token with mint pubkey
    let mut new_mint_instructions = vec![
        create_account(
            &payer.pubkey(),
            &mint_key,
            req
                .get_minimum_balance_for_rent_exemption(82)
                .unwrap(),
            82 ,
            &token_key,
        ),
        initialize_mint(
            &token_key,
            &mint_key,
            &payer.pubkey(),
            Some(&payer.pubkey()),
            0,
        )
        .unwrap(),
    ]; // this contains two instructions create account and intialize mint

    // data
    let temp_data = Data {
        name: name,
        symbol: symbol,
        uri: uri,
        seller_fee_basis_points: 10,
        creators: Some(
            vec![Creator {
            address: payer.pubkey(),
            share: 100,
            verified: false
        }])
    };
    //CreateMetadataAccountArgs
    let update_data = MetadataInstruction::CreateMetadataAccount (
        CreateMetadataAccountArgs {
            data: temp_data,
            is_mutable: true
        }
    ).try_to_vec().unwrap(); // serializing the data 


    let new_metadata_instruction =     Instruction {
        program_id: program_id.clone(),
        accounts: vec![
            AccountMeta::new(metadata_key, false),
            AccountMeta::new_readonly(mint_key, false),
            AccountMeta::new_readonly(payer.pubkey(), true), // payer keypair is used for signing
            AccountMeta::new(wallet.pubkey(), true), // waller keypair is also used for sigining
            AccountMeta::new_readonly(payer.pubkey(), true),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: update_data
    }; // instruction to create metadata account

    let mut instructions = vec![];

    instructions.append(&mut new_mint_instructions);
    instructions.push(new_metadata_instruction);

    // Assign the transction with payer pubkey
    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));

    // get the recent blockhash
    let recent_blockhash = req.get_recent_blockhash().unwrap().0;

    let mut signers = vec![&payer,&wallet, &new_mint]; // we have three keypairs to sign this transaction

    transaction.sign(&signers, recent_blockhash); // sign call

    let sig = req.send_and_confirm_transaction(&transaction).unwrap(); // signature for the transaction

    //println!("\n\n{:?}\n\n",sig);

    let account = req.get_account(&metadata_key).unwrap(); // get account data from created account
    let metadata: Metadata = try_from_slice_unchecked(&account.data).unwrap(); 

    (metadata, metadata_key, mint_key)
}

fn update_metadata_account_test(req: &Client, program_id: &Pubkey, metadata: &Metadata, metadata_key: &Pubkey, mint_key: &Pubkey)
-> (Metadata, Pubkey) {
    let program_info = req.get_account(program_id).unwrap(); // get deployed program information

    if !program_info.executable { // check if it is executable
        println!("Not Executable.");
    }

   // println!("{:?}", program_info );

    let arr: [u8;64] = [
    247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
    195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
    129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
     93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
    144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
    210, 208, 188, 161,  37, 158,   6, 108, 252
  ];
    let payer: Keypair = Keypair::from_bytes(&arr).unwrap(); // deployed program keypair

    let program_key = program_id; // deployed program id

    // Metadata accounts are simply PDA addresses with derived key of `['metaplex', metaplex_program_id, mint_id]`.
    let metadata_seeds = &[PREFIX.as_bytes(), &program_key.as_ref(), mint_key.as_ref()];

    // pub key derived using seed
    let (metadata_key, _) = Pubkey::find_program_address(metadata_seeds, &program_key);

    
    let wall_arr = [244,194,241,225,15,191,246,159,209,33,185,63,66,83,136,245,229,137,231,131,162,22,138,183,202,60,4,145,145,154,181,47,49,225,231,18,254,186,11,157,121,238,174,50,155,59,5,159,240,115,170,215,77,184,187,3,206,62,248,3,130,161,22,169];
    let wallet = Keypair::from_bytes(&wall_arr).unwrap(); // wallet keypair, this pubkey will be our updatedauthority


    let uri = metadata.data.uri.clone(); // uri
    let name = metadata.data.name.clone(); // name
    let new_update_authority = wallet.pubkey(); // palce the new update authority, wallet key in this case
    let update_authority = metadata.update_authority.clone(); // current update authority
    let metadata_account = req.get_account(&metadata_key).unwrap(); // metadata account information
    let metadata: Metadata = try_from_slice_unchecked(&metadata_account.data).unwrap();

    let new_data = Data {
        name: name,
        symbol: metadata.data.symbol,
        uri: uri,
        seller_fee_basis_points: 0,
        creators: metadata.data.creators,
    }; // update data for metadata account, we can update these information too

    let instructions = [
        Instruction {
            program_id: program_id.clone(),
            accounts: vec![
                AccountMeta::new(metadata_key, false),
                AccountMeta::new_readonly(update_authority, true), // payer is the signer
            ],
            data: MetadataInstruction::UpdateMetadataAccount(UpdateMetadataAccountArgs {
                data: Some(new_data),
                update_authority: Some(new_update_authority), // new update authority
                primary_sale_happened: None,
            })
            .try_to_vec()
            .unwrap(),
        }
    ];

    // sign the tranaction with payer pubkey
    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));

    // get recent blockhash
    let recent_blockhash = req.get_recent_blockhash().unwrap().0;

    // this instruction needs only one signer, as payer is our current update authority
    let mut signers = vec![&payer];

    transaction.sign(&signers, recent_blockhash); // sign call

    let sig = req.send_and_confirm_transaction(&transaction).unwrap(); // signature

   // println!("\n\n{:?}\n\n",sig);

    let metadata_account = req.get_account(&metadata_key).unwrap(); // get the update data information from the account
    let metadata: Metadata = try_from_slice_unchecked(&metadata_account.data).unwrap();
    (metadata, metadata_key)
}

fn create_master_edition_test(req: &Client, program_id: &Pubkey, mint_key: &Pubkey) -> (MasterEditionV2, Pubkey, Keypair) {
    let arr: [u8;64] = [
        247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
        195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
        129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
         93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
        144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
        210, 208, 188, 161,  37, 158,   6, 108, 252
      ];
    let payer: Keypair = Keypair::from_bytes(&arr).unwrap(); // deployed program keypair

    let program_key = program_id;
    let token_key = Pubkey::from_str(TOKEN_PROGRAM_PUBKEY).unwrap();

    let update_authority = &payer; // update_authority for our mint
    let mint_authority = &payer; // mint authority for our mint

    let metadata_seeds = &[PREFIX.as_bytes(), &program_key.as_ref(), mint_key.as_ref()]; // seed for metadata account
    let (metadata_key, _) = Pubkey::find_program_address(metadata_seeds, &program_key); // get pubkey for metadata account

    let metadata_account = req.get_account(&metadata_key).unwrap();
    let metadata: Metadata = try_from_slice_unchecked(&metadata_account.data).unwrap(); // get data from metadata account

    let master_edition_seeds = &[ // seed for master edition that we will create
        PREFIX.as_bytes(),
        &program_key.as_ref(),
        &metadata.mint.as_ref(),
        EDITION.as_bytes()
    ]; 
                                                // Pubkey found using the masteredition seed
    let (master_edition_key, _) = Pubkey::find_program_address(master_edition_seeds, &program_key);

    let max_supply = 10; // max supply 10

    let added_token_account = Keypair::new(); // new key pair for token account
    

    let mut signers = vec![update_authority, mint_authority]; // all signers
    signers.push(&added_token_account);

    let mut instructions = vec![];

    let create_token_account_ins = create_account(
        &payer.pubkey(),
        &added_token_account.pubkey(),
        req
            .get_minimum_balance_for_rent_exemption(165)
            .unwrap(),
        165,
        &token_key,
    );

    let initialize_account_ins = initialize_account(
        &token_key,
        &added_token_account.pubkey(),
        &metadata.mint,
        &payer.pubkey(),
    )
    .unwrap();

    let mint_to_ins = mint_to(
        &token_key,
        &metadata.mint,
        &added_token_account.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        1,
    )
    .unwrap();

    instructions.push(create_token_account_ins);
    instructions.push(initialize_account_ins);
    instructions.push(mint_to_ins);

    let accounts = vec![
        AccountMeta::new(master_edition_key, false),
        AccountMeta::new(mint_key.clone(), false),
        AccountMeta::new_readonly(update_authority.pubkey(), true),
        AccountMeta::new_readonly(mint_authority.pubkey(), true),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new_readonly(metadata_key, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    let create_master_edition_ins = Instruction {
        program_id: program_key.clone(),
        accounts,
        data: MetadataInstruction::CreateMasterEdition(CreateMasterEditionArgs { max_supply: Some(max_supply) })
        .try_to_vec()
        .unwrap()
    };

    instructions.push(create_master_edition_ins);
    
    // sign the transaction with payer
    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));

    //get recent block hash
    let recent_blockhash = req.get_recent_blockhash().unwrap().0;

    transaction.sign(&signers, recent_blockhash); // sign call

    req.send_and_confirm_transaction(&transaction).unwrap();

    let account = req.get_account(&master_edition_key).unwrap();
    let master_edition: MasterEditionV2 = try_from_slice_unchecked(&account.data).unwrap();
    (master_edition, master_edition_key, added_token_account)


}

fn mint_edition_via_token_call(req: &Client, program_key: &Pubkey, mint_key: &Pubkey, token_account: &Pubkey) 
-> (Edition, Pubkey, Pubkey) {
    let arr: [u8;64] = [
        247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
        195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
        129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
         93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
        144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
        210, 208, 188, 161,  37, 158,   6, 108, 252
      ];
    let payer: Keypair = Keypair::from_bytes(&arr).unwrap(); // deployed program keypair

    let account_authority = &payer;
    let token_key = Pubkey::from_str(TOKEN_PROGRAM_PUBKEY).unwrap();

    let existing_token_account = token_account;

    let new_mint_key = Keypair::new();
    let added_token_account = Keypair::new();
    let new_mint_pub = new_mint_key.pubkey();
    let metadata_seeds = &[
        PREFIX.as_bytes(),
        &program_key.as_ref(),
        &new_mint_pub.as_ref(),
    ];
    let (metadata_key, _) = Pubkey::find_program_address(metadata_seeds, &program_key);

    let edition_seeds = &[
        PREFIX.as_bytes(),
        &program_key.as_ref(),
        &new_mint_pub.as_ref(),
        EDITION.as_bytes(),
    ];
    let (edition_key, _) = Pubkey::find_program_address(edition_seeds, &program_key);

    let master_metadata_seeds = &[PREFIX.as_bytes(), &program_key.as_ref(), mint_key.as_ref()];
    let (master_metadata_key, _) =
        Pubkey::find_program_address(master_metadata_seeds, &program_key);

    let master_metadata_account = req.get_account(&master_metadata_key).unwrap();
    let master_metadata: Metadata =
        try_from_slice_unchecked(&master_metadata_account.data).unwrap();

    let master_edition_seeds = &[
        PREFIX.as_bytes(),
        &program_key.as_ref(),
        &master_metadata.mint.as_ref(),
        EDITION.as_bytes(),
    ];
    let (master_edition_key, _) = Pubkey::find_program_address(master_edition_seeds, &program_key);
    let master_edition_account = req.get_account(&master_edition_key).unwrap();
    let master_edition: MasterEditionV2 =
        try_from_slice_unchecked(&master_edition_account.data).unwrap();

    let signers = vec![account_authority, &new_mint_key, &added_token_account];
    let mut instructions = vec![
        create_account(
            &payer.pubkey(),
            &new_mint_key.pubkey(),
            req
                .get_minimum_balance_for_rent_exemption(82)
                .unwrap(),
            82,
            &token_key,
        ),
        initialize_mint(
            &token_key,
            &new_mint_key.pubkey(),
            &payer.pubkey(),
            Some(&payer.pubkey()),
            0,
        )
        .unwrap(),
        create_account(
            &payer.pubkey(),
            &added_token_account.pubkey(),
            req
                .get_minimum_balance_for_rent_exemption(165)
                .unwrap(),
            165,
            &token_key,
        ),
        initialize_account(
            &token_key,
            &added_token_account.pubkey(),
            &new_mint_key.pubkey(),
            &payer.pubkey(),
        )
        .unwrap(),
        mint_to(
            &token_key,
            &new_mint_key.pubkey(),
            &added_token_account.pubkey(),
            &payer.pubkey(),
            &[&payer.pubkey()],
            1,
        )
        .unwrap(),
    ];
    let edition = master_edition.supply + 1;
    let edition_number = edition.checked_div(248).unwrap();
    let as_string = edition_number.to_string();
    let (edition_mark_pda, _) = Pubkey::find_program_address(
        &[
            PREFIX.as_bytes(),
            program_key.as_ref(),
            master_metadata.mint.as_ref(),
            EDITION.as_bytes(),
            as_string.as_bytes(),
        ],
        program_key,
    );

    let accounts = vec![
        AccountMeta::new(metadata_key, false),
        AccountMeta::new(edition_key, false),
        AccountMeta::new(master_edition_key, false),
        AccountMeta::new(new_mint_key.pubkey(), false),
        AccountMeta::new(edition_mark_pda, false),
        AccountMeta::new_readonly(account_authority.pubkey(), true),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new_readonly(account_authority.pubkey(), true),
        AccountMeta::new_readonly(existing_token_account.clone(), false),
        AccountMeta::new_readonly(account_authority.pubkey(), false),
        AccountMeta::new_readonly(master_metadata_key, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    let ins = Instruction {
        program_id: program_key.clone(),
        accounts,
        data: MetadataInstruction::MintNewEditionFromMasterEditionViaToken(
            MintNewEditionFromMasterEditionViaTokenArgs { edition },
        )
        .try_to_vec()
        .unwrap(),
    };

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
    let recent_blockhash = req.get_recent_blockhash().unwrap().0;

    transaction.sign(&signers, recent_blockhash);
    req.send_and_confirm_transaction(&transaction).unwrap();
    let account = req.get_account(&edition_key).unwrap();
    let edition: Edition = try_from_slice_unchecked(&account.data).unwrap();
    (edition, edition_key, new_mint_key.pubkey())
}



fn main() {
    let req = establish_connection();
    let program_id = Pubkey::from_str("CFzwMLnq9GgLCfqiV2LGhTLqsdNsKKGv2A82AtD5FZo4").unwrap();


    // We will have a random mint token asset, we have to create a metadata account for that mint
    // we will create a metadata account(PDA) using ['metadata', program id, mint id]
    // our metadata will will have values for Data struct

    let (metadata, metadata_key, mint_key) =  create_metadata_account_test(&req, &program_id);
    println!("Current Update Authoriry for mint {:?} is {:?}\n",metadata.mint, metadata.update_authority);
    
    //  let (metadata, metadata_key) = update_metadata_account_test(&req, &program_id, &metadata, &metadata_key, &mint_key);
    //  println!("Updated Authority for mint {:?} is {:?}\n",metadata.mint, metadata.update_authority);


    // we have a mint 5xsFN6FoSeDE7BTgjQWoUbfvUEHHTsrmmy7h3opZ2Got, its update authority is 6YX3wuJixYZ35xrN7wTbBiJVB8p4pPbVuCZjLUqhx2C3
    // we are creating a master edition for this mint 5xsFN6FoSeDE7BTgjQWoUbfvUEHHTsrmmy7h3opZ2Got
    // we create a token account E91dBeDhtBZ2vZ9CEAQdQi4fvFAyKHZbFCixwU4sqvY1 that will hold this mint 5xsFN6FoSeDE7BTgjQWoUbfvUEHHTsrmmy7h3opZ2Got
    // now we update the mint authority, freeze authroity of our mint to a master edition key(PDA) 3VEJjCCaCfkJuPfzhUa5iLPg4jUYtxgGBYHMLdpPuunE
    let (master_edition, master_edition_key, token_account_key) = create_master_edition_test(&req, &program_id, &mint_key);

    println!("Master Edition: {:?}, Key: {:?}, Token account: {:?}", master_edition.max_supply, master_edition_key, token_account_key.pubkey());
}