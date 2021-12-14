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
/// Args for create call
pub struct CreateMetadataAccountArgs {
    /// Note that unique metadatas are disabled for now.
    pub data: Data,
    /// Whether you want your metadata to be updateable in the future.
    pub is_mutable: bool,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct testargs {
    pub val: UpdateMetadataAccountArgs
}
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct testcreateargs {
    pub val: CreateMetadataAccountArgs
}

/// Instructions supported by the Metadata program.
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub enum MetadataInstruction {
    UpdateMetadataAccount(UpdateMetadataAccountArgs),
    testdata(testargs),
    CreateMetadataAccount(CreateMetadataAccountArgs),
    testcreate(testcreateargs)
    // UpdateMetadataAccount: UpdateMetadataAccountArgs
}




fn check_program(client: &Client, payer: &Pubkey, pro_pubkey: &Pubkey) -> Pubkey {
    let program_info = client.get_account(pro_pubkey).unwrap();
    if !program_info.executable {
        println!("not executable");
    }
    println!("Using program id: {}",pro_pubkey);

    let wall_arr = [244,194,241,225,15,191,246,159,209,33,185,63,66,83,136,245,229,137,231,131,162,22,138,183,202,60,4,145,145,154,181,47,49,225,231,18,254,186,11,157,121,238,174,50,155,59,5,159,240,115,170,215,77,184,187,3,206,62,248,3,130,161,22,169];

    let wallet = Keypair::from_bytes(&wall_arr).unwrap();

    println!("{} ",wallet.pubkey());

    let payer = &wallet.pubkey();

    println!("creating metadata account using seed");
    let program_key = pro_pubkey.clone();
   // let token_key = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    let name = String::from("test 1");
    let symbol = String::from("test 2");
    let uri = String::from("https://arweave.net/WcQTJhsilu0gMOVCHxI5Q3WqGkyLhnb6grmVCE7NLH4");
    let create_new_mint = false;
    let mutable = true;
    let new_mint = Keypair::new();
    let mint_key = Pubkey::from_str("HXUVD9PFbWfnAjm8bU2SCpPmjjrV3iSjqC4kyz9snneh").unwrap();
    let metadata_seeds = &[PREFIX.as_bytes(), &program_key.as_ref(), mint_key.as_ref()];
    pub const PREFIX: &str = "metadata";
    let (metadata_key, _) = Pubkey::find_program_address(metadata_seeds, &program_key);
    println!("all set");
    // let mut new_mint_instructions = vec![
    //     create_account(
    //         &payer.pubkey(),
    //         &mint_key,
    //         client
    //             .get_minimum_balance_for_rent_exemption(Mint::LEN)
    //             .unwrap(),
    //         Mint::LEN as u64,
    //         &token_key,
    //     ),
    //     initialize_mint(
    //         &token_key,
    //         &mint_key,
    //         &payer.pubkey(),
    //         Some(&payer.pubkey()),
    //         0,
    //     )
    //     .unwrap(),
    // ];

    // let new_metadata_instruction = create_metadata_accounts(
    //     program_key,
    //     metadata_key,
    //     mint_key,
    //     payer.clone(),
    //     payer.clone(),
    //     wallet.pubkey(),
    //     name,
    //     symbol,
    //     uri,
    //     None,
    //     0,
    //     wallet.pubkey() != payer.clone(),
    //     mutable,
    // );
    let temp_data = Data {
        name: String::from("kaines"),
        symbol: String::from("kaines sym"),
        uri: String::from("kaines"),
        seller_fee_basis_points: 1,
        creators: Some(
            vec![Creator {
            address: payer.clone(),
            share: 10,
            verified: true
        }])
    };
    let update_data = MetadataInstruction::testcreate(
        testcreateargs {
            val: CreateMetadataAccountArgs {
                data: temp_data,
                is_mutable: true,
                }
        }
    ).try_to_vec().unwrap();

    let temp_data = Data {
        name: String::from("kaines"),
        symbol: String::from("kaines sym"),
        uri: String::from("kaines"),
        seller_fee_basis_points: 1,
        creators: Some(
            vec![Creator {
            address: payer.clone(),
            share: 10,
            verified: true
        }])
    };
    let update_data = MetadataInstruction::UpdateMetadataAccount (
        UpdateMetadataAccountArgs {
            data: Some(temp_data),
            update_authority: None,
            primary_sale_happened: None
        }
    ).try_to_vec().unwrap();
    println!("{:?}", update_data);

    let test_deserialize: MetadataInstruction = MetadataInstruction::try_from_slice(&update_data).unwrap();
    println!("\n\n\n\n{:?}\n\n\n\n",test_deserialize);


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
    //                         address: payer.clone(),
    //                         share: 10,
    //                         verified: true
    //                     }])
    //                 }
    //             ),
    //             update_authority: None,
    //             primary_sale_happened: Some(true)
    //         }
    //     }
    // ).try_to_vec().unwrap();

    let new_metadata_instruction =     Instruction {
        program_id: program_key,
        accounts: vec![
            AccountMeta::new(metadata_key, false),
            AccountMeta::new_readonly(mint_key, false),
            AccountMeta::new_readonly(payer.clone(), true),
            AccountMeta::new(payer.clone(), true),
            AccountMeta::new_readonly(wallet.pubkey(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: update_data
    };

    let mut instructions = vec![];

    // if create_new_mint {
    //     instructions.append(&mut new_mint_instructions)
    // }

    instructions.push(new_metadata_instruction);


    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.clone()));
    println!("\n\n\n{:?}\n\n\n",transaction);
    let recent_blockhash = client.get_recent_blockhash().unwrap().0;

    let mut signers = vec![&wallet];
    // if create_new_mint {
    //     signers.push(&new_mint);
    // }
    // if wallet.pubkey() != payer.pubkey() {
    //     signers.push(&wallet)
    // }
    transaction.sign(&signers, recent_blockhash);
    client.send_and_confirm_transaction(&transaction).unwrap();

    return metadata_key;
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

    let wall_arr = [244,194,241,225,15,191,246,159,209,33,185,63,66,83,136,245,229,137,231,131,162,22,138,183,202,60,4,145,145,154,181,47,49,225,231,18,254,186,11,157,121,238,174,50,155,59,5,159,240,115,170,215,77,184,187,3,206,62,248,3,130,161,22,169];

    let wallet = Keypair::from_bytes(&wall_arr).unwrap();

    println!("{} ",wallet.pubkey());


    let arr = [183,212,60,41,77,203,242,226,243,125,211,169,111,24,159,184,5,169,54,110,177,128,157,168,210,218,84,228,140,254,132,121,15,219,118,81,151,250,201,126,110,198,196,9,32,245,61,146,31,89,179,145,224,240,197,69,90,39,105,146,160,180,3,164];

    let payer2 = Keypair::from_bytes(&arr).unwrap(); //4MimTvfcxafduVjcoLAPnQ8QZnqkHQBM373cJKj5WdMA


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
    let just_data = Data {
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
    };
    

    let pro_pubkey: Pubkey = Pubkey::from_str("24uD7rNTrzraSnKJvkwmDcXVKRxhv9DdYijBfbVXNSE7").unwrap();
    // let instruct = instruction::Instruction::new_with_borsh(pro_pubke, &update_data, keys);
    let instruct =     Instruction {
        program_id: pro_pubkey.clone(),
        accounts: vec![
            AccountMeta::new(metadata_account.clone(), false),
            AccountMeta::new_readonly(wallet.pubkey().clone(), true),
        ],
        data: MetadataInstruction::testdata(
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
            }).try_to_vec().unwrap(),
    };
    
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

    println!("{:?}\n\n",inst.try_sign(&[&payer,&wallet], blockhash));
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
    let payer = Pubkey::from_str("4MimTvfcxafduVjcoLAPnQ8QZnqkHQBM373cJKj5WdMA").unwrap();
    let pro_pubkey = Pubkey::from_str("24uD7rNTrzraSnKJvkwmDcXVKRxhv9DdYijBfbVXNSE7").unwrap();

    let metadata_account = check_program(&client, &payer, &pro_pubkey);

    update_metadata(&client, &metadata_account, &pro_pubkey, &payer);
}
