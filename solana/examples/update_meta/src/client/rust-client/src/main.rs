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
use solana_program::{pubkey, system_instruction,};
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
   //let client = RpcClient::new_with_commitment("http://localhost:8899".into(), CommitmentConfig::confirmed());
    let client = Client { client, payer };
    println!("Connection established {:?}\n",&client.get_version());
    client
}


fn check_program(client: &Client, payer: &Pubkey, pro_pubkey: &Pubkey) -> Pubkey {
    let program_info = client.get_account(pro_pubkey).unwrap();
    if !program_info.executable {
        println!("not executable");
    }
    println!("Using program id: {}",pro_pubkey);

    println!("creating metadata account using seed");

    // let tempdata = Data {
    //     name: String::from("kaines"),
    //     symbol: String::from("thisis"),
    //     uri: String::from("https://arweave.net/dv98rlugi0HesC6aMdZ3dDl8iUPSqpmblsuLlzVWgkM"),
    //     seller_fee_basis_points: 1,
    //     creators: Some(vec![
    //         Creator {
    //             address: payer.clone(),
    //             verified: true,
    //             share:100,
    //         }
    //     ])
    // };
    // let data = tempdata.try_to_vec().unwrap();

    // let arr = [183,212,60,41,77,203,242,226,243,125,211,169,111,24,159,184,5,169,54,110,177,128,157,168,210,218,84,228,140,254,132,121,15,219,118,81,151,250,201,126,110,198,196,9,32,245,61,146,31,89,179,145,224,240,197,69,90,39,105,146,160,180,3,164];

    // let payer2 = Keypair::from_bytes(&arr).unwrap();

    // let update_auth = Pubkey::from_str("6YX3wuJixYZ35xrN7wTbBiJVB8p4pPbVuCZjLUqhx2C3").unwrap();
    // let form_key = AccountMeta::new(update_auth.clone(), true);
    // let to_key = AccountMeta::new_readonly(payer2.pubkey().clone(), true);

    // let mut keys = Vec::new();

    // keys.push(form_key);
    // keys.push(to_key);



    let seed = "Micheal";
    let pda = Pubkey::create_with_seed(payer, seed, pro_pubkey).unwrap();
    // let create_account_instruction = system_instruction::create_account_with_seed(payer, &pda, pro_pubkey, "Micheal", 100, mem::size_of_val(&tempdata) as u64, payer);

    // let instruction = Instruction::new_with_bytes(*payer, &data, keys);
    // println!("hello_instruction: {:?}", create_account_instruction);
    // let ins_vec = vec![create_account_instruction,instruction];

    // let mut inst = Transaction::new_with_payer(&ins_vec, Some(&payer2.pubkey()));

    // println!("{:?}\n\n",inst);
    // let arr = [
    //     247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
    //     195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
    //     129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
    //      93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
    //     144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
    //     210, 208, 188, 161,  37, 158,   6, 108, 252
    //   ];
    // let payer1 = Keypair::from_bytes(&arr).unwrap();
    // let blockhash = client.get_recent_blockhash().unwrap().0;



    // println!("{:?}\n\n",inst.try_sign(&[&payer1, &payer2, &payer1], blockhash));
    // let sig = client.send_and_confirm_transaction(&inst);

    // println!("{:?}",sig);
    println!("Our Current metadata account: {}", pda);

    return pda;
}
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    // In percentages, NOT basis points ;) Watch out!
    pub share: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Data {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    /// Array of creators, optional
    pub creators: Option<Vec<Creator>>,
}
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct UpdateMetadataAccountArgs {
    pub data: Option<Data>,
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct testargs {
    pub val: UpdateMetadataAccountArgs
}



#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub enum MetadataInstruction {
    UpdateMetadataAccount(UpdateMetadataAccountArgs),
    testdata(testargs)
    // UpdateMetadataAccount: UpdateMetadataAccountArgs
}

fn update_metadata(client: &Client ,metadata_account: &Pubkey, pro_pubkey: &Pubkey, pay_pubkey: &Pubkey) {
    // let meta_key = AccountMeta::new(metadata_account.clone(), true);
    // let update_auth = Pubkey::from_str("6YX3wuJixYZ35xrN7wTbBiJVB8p4pPbVuCZjLUqhx2C3").unwrap();
    // let update_auth_key = AccountMeta::new(update_auth.clone(), true);
    // let vault_key = Pubkey::from_str("4MimTvfcxafduVjcoLAPnQ8QZnqkHQBM373cJKj5WdMA").unwrap();
    // let mut keys = Vec::new();
    // keys.push(meta_key);
    // keys.push(update_auth_key);
    // let tempdata = Data {
    //     name: String::from("kaines"),
    //     symbol: String::from("thisis"),
    //     uri: String::from("https://arweave.net/dv98rlugi0HesC6aMdZ3dDl8iUPSqpmblsuLlzVWgkM"),
    //     seller_fee_basis_points: 1,
    //     creators: Some(vec![
    //         Creator {
    //             address: update_auth.clone(),
    //             verified: true,
    //             share:100,
    //             }
    //     ])
    // };
    // let update_data = MetadataInstruction::UpdateMetadataAccount( UpdateMetadataAccountArgs {
    //     data: Some(tempdata),
    //     update_authority: Some(update_auth.clone()),
    //     primary_sale_happened: Some(true)
    // });


    // let arr = [183,212,60,41,77,203,242,226,243,125,211,169,111,24,159,184,5,169,54,110,177,128,157,168,210,218,84,228,140,254,132,121,15,219,118,81,151,250,201,126,110,198,196,9,32,245,61,146,31,89,179,145,224,240,197,69,90,39,105,146,160,180,3,164];

    // let payer2 = Keypair::from_bytes(&arr).unwrap();
    // let data = update_data;
    // // let test = MetadataInstruction::try_from_slice(&data).unwrap();
    // // println!("\n\n\n{:?}\n\n\n",test);
    // // match test {
    // //     // this is updatemetadataacout impl
    // //     MetadataInstruction::UpdateMetadataAccount(args) => {
    // //         println!("working");
    // //     }
    // // }
    // let mut ins_vec = vec![];
    // let pro_pubke: Pubkey = Pubkey::from_str("24uD7rNTrzraSnKJvkwmDcXVKRxhv9DdYijBfbVXNSE7").unwrap();
    // let instruct = instruction::Instruction::new_with_borsh(pro_pubke, &data, vec![]);
    // ins_vec.push(instruct);
    // //println!("{:?}\n\n",ins_vec);
    // let mut inst = Transaction::new_with_payer(&ins_vec, Some(&pay_pubkey));

    // println!("{:?}\n\n",inst);
    // let arr = [
    //     247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
    //     195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
    //     129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
    //      93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
    //     144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
    //     210, 208, 188, 161,  37, 158,   6, 108, 252
    //   ];
    // let payer = Keypair::from_bytes(&arr).unwrap();

    // let blockhash = client.get_recent_blockhash().unwrap().0;

    // println!("{:?}\n\n",inst.try_sign(&[&payer], blockhash));
    // let sig = client.send_and_confirm_transaction(&inst);

    // println!("{:?}\n\n",sig);

        // let arr = [183,212,60,41,77,203,242,226,243,125,211,169,111,24,159,184,5,169,54,110,177,128,157,168,210,218,84,228,140,254,132,121,15,219,118,81,151,250,201,126,110,198,196,9,32,245,61,146,31,89,179,145,224,240,197,69,90,39,105,146,160,180,3,164];

    // let payer2 = Keypair::from_bytes(&arr).unwrap();


    // let key = AccountMeta::new(pay_pubkey.clone(), false);
    // let mut keys = Vec::new();
    // keys.push(key);
    // let update_data = MetadataInstruction::testdata(
    //     testargs {
    //         val: UpdateMetadataAccountArgs {
    //             data: Some(
    //                 Data {
    //                     name: String::from("kaines"),
    //                     symbol: String::from("kaines sym"),
    //                     uri: String::from("kaines"),
    //                     seller_fee_basis_points: 10,
    //                     creators: Some(
    //                         vec![Creator {
    //                         address: pay_pubkey.clone(),
    //                         share: 10,
    //                         verified: true
    //                     }])
    //                 }
    //             ),
    //             update_authority: None,
    //             primary_sale_happened: Some(true)
    //         }
    //     }
    // );


    // let pro_pubke: Pubkey = Pubkey::from_str("24uD7rNTrzraSnKJvkwmDcXVKRxhv9DdYijBfbVXNSE7").unwrap();
    // let instruct = instruction::Instruction::new_with_borsh(pro_pubke, &update_data, keys);
    
    // let mut ins_vec = vec![];
    // ins_vec.push(instruct);
    // println!("{:?}\n\n",ins_vec);
    // let mut inst = Transaction::new_with_payer(&ins_vec, Some(&client.payer_pubkey()));

    // println!("{:?}",inst);
    // let arr = [
    //     247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
    //     195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
    //     129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
    //      93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
    //     144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
    //     210, 208, 188, 161,  37, 158,   6, 108, 252
    //   ];
    // let payer = Keypair::from_bytes(&arr).unwrap();
    // let blockhash = client.get_recent_blockhash().unwrap().0;

    // println!("{:?}\n\n",inst.try_sign(&[&payer], blockhash));
    // let sig = client.send_and_confirm_transaction(&inst);

    // println!("{:?}\n\n",sig);


    let arr = [183,212,60,41,77,203,242,226,243,125,211,169,111,24,159,184,5,169,54,110,177,128,157,168,210,218,84,228,140,254,132,121,15,219,118,81,151,250,201,126,110,198,196,9,32,245,61,146,31,89,179,145,224,240,197,69,90,39,105,146,160,180,3,164];

    let payer2 = Keypair::from_bytes(&arr).unwrap();


    let key1 = AccountMeta::new(metadata_account.clone(), false);
    let key2 = AccountMeta::new_readonly(pay_pubkey.clone(), true);
    let mut keys = Vec::new();
    keys.push(key1);
    keys.push(key2);
    let update_data = MetadataInstruction::testdata(
        testargs {
            val: UpdateMetadataAccountArgs {
                data: Some(
                    Data {
                        name: String::from("kaines"),
                        symbol: String::from("kaines sym"),
                        uri: String::from("kaines"),
                        seller_fee_basis_points: 10,
                        creators: Some(
                            vec![Creator {
                            address: pay_pubkey.clone(),
                            share: 10,
                            verified: true
                        }])
                    }
                ),
                update_authority: None,
                primary_sale_happened: Some(true)
            }
        }
    );


    let pro_pubke: Pubkey = Pubkey::from_str("24uD7rNTrzraSnKJvkwmDcXVKRxhv9DdYijBfbVXNSE7").unwrap();
    let instruct = instruction::Instruction::new_with_borsh(pro_pubke, &update_data, keys);
    
    let mut ins_vec = vec![];
    ins_vec.push(instruct);
    println!("{:?}\n\n",ins_vec);
    let mut inst = Transaction::new_with_payer(&ins_vec, Some(&client.payer_pubkey()));

    println!("{:?}",inst);
    let arr = [
        247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
        195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
        129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
         93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
        144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
        210, 208, 188, 161,  37, 158,   6, 108, 252
      ];
    let payer = Keypair::from_bytes(&arr).unwrap();
    let blockhash = client.get_recent_blockhash().unwrap().0;

    println!("{:?}\n\n",inst.try_sign(&[&payer], blockhash));
    let sig = client.send_and_confirm_transaction(&inst);

    println!("{:?}\n\n",sig);

    


    // let arr = [183,212,60,41,77,203,242,226,243,125,211,169,111,24,159,184,5,169,54,110,177,128,157,168,210,218,84,228,140,254,132,121,15,219,118,81,151,250,201,126,110,198,196,9,32,245,61,146,31,89,179,145,224,240,197,69,90,39,105,146,160,180,3,164];

    // let payer2 = Keypair::from_bytes(&arr).unwrap();

    // // Account Meta keys
    // let meta_key = AccountMeta::new(payer2.pubkey().clone(), true);
    // let update_auth = Pubkey::from_str("6YX3wuJixYZ35xrN7wTbBiJVB8p4pPbVuCZjLUqhx2C3").unwrap();
    // let update_auth_key = AccountMeta::new(update_auth.clone(), false);

    // let mut keys = Vec::new();
    // keys.push(meta_key);
    // keys.push(update_auth_key);

   

    // let update_data = MetadataInstruction::UpdateMetadataAccount (
    //     UpdateMetadataAccountArgs {
    //         data: Some(
    //             Data {
    //                 name: String::from("kaines"),
    //                 symbol: String::from("kaines sym"),
    //                 uri: String::from("kaines"),
    //                 seller_fee_basis_points: 10,
    //                 creators: Some(
    //                     vec![Creator {
    //                     address: pay_pubkey.clone(),
    //                     share: 10,
    //                     verified: true
    //                 }])
    //             }
    //         ),
    //         update_authority: None,
    //         primary_sale_happened: Some(true)
    //     }
    // );


    // let pro_pubke: Pubkey = Pubkey::from_str("24uD7rNTrzraSnKJvkwmDcXVKRxhv9DdYijBfbVXNSE7").unwrap();
    // let instruct = instruction::Instruction::new_with_borsh(pro_pubke, &update_data, keys);
    
    // let mut ins_vec = vec![];
    // ins_vec.push(instruct);
    // println!("{:?}\n\n",ins_vec);
    // let mut inst = Transaction::new_with_payer(&ins_vec, Some(&client.payer_pubkey()));

    // println!("{:?}",inst);
    // let arr = [
    //     247,  44, 145,  42, 156, 254, 127, 211, 249, 218, 142,
    //     195, 158, 119, 118, 133,  54,  36, 158,  80, 103, 146,
    //     129,  53, 159, 226, 228, 108,  26, 179, 247,  37,  82,
    //      93, 107,  19,  98, 150,  38, 212,  20, 130,  43, 169,
    //     144, 206, 245,  52, 188, 191,   5,  69,   9,  14,  47,
    //     210, 208, 188, 161,  37, 158,   6, 108, 252
    //   ];
    // let payer = Keypair::from_bytes(&arr).unwrap();
    // let blockhash = client.get_recent_blockhash().unwrap().0;

    // println!("{:?}\n\n",inst.try_sign(&[&payer, &payer2], blockhash));
    // let sig = client.send_and_confirm_transaction(&inst);

    // println!("{:?}\n\n",sig);



}



fn main() {
    println!("Start");
    let client = establish_connection();
    let payer = Pubkey::from_str("6YX3wuJixYZ35xrN7wTbBiJVB8p4pPbVuCZjLUqhx2C3").unwrap();
    let pro_pubkey = Pubkey::from_str("24uD7rNTrzraSnKJvkwmDcXVKRxhv9DdYijBfbVXNSE7").unwrap();

    let metadata_account = check_program(&client, &payer, &pro_pubkey);

    update_metadata(&client, &metadata_account, &pro_pubkey, &payer);
}
