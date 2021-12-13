use client::instruction::{ MetadataInstruction, Test1Args, Test2Args };
use client::state::{ Data, Creator };

use std::str::FromStr;
use borsh::{BorshDeserialize, BorshSerialize};
use std::mem;
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
use solana_program::{pubkey, system_instruction, sysvar};
use solana_client::rpc_client::RpcClient;
use solana_client_helpers::{Client, ClientResult, SplToken};
use std::{process::exit, sync::Arc};



fn establish_connection() -> Client{
    let arr = [
        247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
        195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
        129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
         93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
        144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
        210, 208, 188, 161,  37, 158,   6, 108, 252
      ];
    let payer = Keypair::from_bytes(&arr).unwrap();
    
    let client = RpcClient::new_with_commitment("https://api.devnet.solana.com".into(), CommitmentConfig::confirmed());
  // let client = RpcClient::new_with_commitment("http://localhost:8899".into(), CommitmentConfig::confirmed());
    let client = Client { client, payer };
    println!("Connection established {:?}\n",&client.get_version());
    client
}

fn check_test1(req: &Client, program_id: &Pubkey) {
    let program_info = req.get_account(program_id).unwrap();

    if !program_info.executable {
        println!("Not Executable.");
    }

    println!("{:?}", program_info );

    let arr: [u8;64] = [
    247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
    195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
    129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
     93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
    144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
    210, 208, 188, 161,  37, 158,   6, 108, 252
  ];
    let payer: Keypair = Keypair::from_bytes(&arr).unwrap();

    let temp_data = Data {
        name: String::from("kaines"),
        symbol: String::from("kaines sym"),
        uri: String::from("kaines"),
        seller_fee_basis_points: 1,
        creators: Some(
            vec![Creator {
            address: payer.pubkey(),
            share: 10,
            verified: true
        }])
    };
    let update_data = MetadataInstruction::Test1 (
        Test1Args {
            data: temp_data,
            is_mutable: true
        }
    ).try_to_vec().unwrap();
    println!("{:?}", update_data);

    let _test_deserialize: MetadataInstruction = MetadataInstruction::try_from_slice(&update_data).unwrap();


    //println!("\n\n\n\n{:?}\n\n\n\n",test_deserialize.data);


    let new_metadata_instruction =     Instruction {
        program_id: program_id.clone(),
        accounts: vec![
        ],
        data: update_data
    };

    let mut instructions = vec![];

    instructions.push(new_metadata_instruction);


    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
    println!("\n\n\n{:?}\n\n\n",transaction);
    let recent_blockhash = req.get_recent_blockhash().unwrap().0;

    let mut signers = vec![&payer];

    transaction.sign(&signers, recent_blockhash);
    let sig = req.send_and_confirm_transaction(&transaction).unwrap();

    println!("\n\n{:?}\n\n",sig);


}

fn check_test2(req: &Client, program_id: &Pubkey) {
    let program_info = req.get_account(program_id).unwrap();

    if !program_info.executable {
        println!("Not Executable.");
    }

    println!("{:?}", program_info );

    let arr: [u8;64] = [
    247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
    195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
    129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
     93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
    144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
    210, 208, 188, 161,  37, 158,   6, 108, 252
  ];
    let payer: Keypair = Keypair::from_bytes(&arr).unwrap();

    let temp_data = Data {
        name: String::from("kaines"),
        symbol: String::from("kaines sym"),
        uri: String::from("kaines"),
        seller_fee_basis_points: 1,
        creators: Some(
            vec![Creator {
            address: payer.pubkey(),
            share: 10,
            verified: true
        }])
    };
    let update_data = MetadataInstruction::Test2 (
        Test2Args {
            data: Some(temp_data),
            update_authority: None,
            primary_sale_happened: None
        }
    ).try_to_vec().unwrap();
    println!("{:?}", update_data);

    let _test_deserialize: MetadataInstruction = MetadataInstruction::try_from_slice(&update_data).unwrap();


    //println!("\n\n\n\n{:?}\n\n\n\n",test_deserialize.data);


    let new_metadata_instruction =     Instruction {
        program_id: program_id.clone(),
        accounts: vec![
        ],
        data: update_data
    };

    let mut instructions = vec![];

    instructions.push(new_metadata_instruction);


    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
    println!("\n\n\n{:?}\n\n\n",transaction);
    let recent_blockhash = req.get_recent_blockhash().unwrap().0;

    let mut signers = vec![&payer];

    transaction.sign(&signers, recent_blockhash);
    let sig = req.send_and_confirm_transaction(&transaction).unwrap();

    println!("\n\n{:?}\n\n",sig);


}



fn main() {
    let req = establish_connection();
    let program_id = Pubkey::from_str("CFzwMLnq9GgLCfqiV2LGhTLqsdNsKKGv2A82AtD5FZo4").unwrap();
    check_test1(&req, &program_id);
    check_test2(&req, &program_id);
}