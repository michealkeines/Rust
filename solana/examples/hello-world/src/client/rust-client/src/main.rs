use solana_program::{
    pubkey::Pubkey
};
use std::str::FromStr;
use  solana_program::account_info::Account;
use solana_program::instruction::AccountMeta;
use solana_sdk::transaction::Transaction;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction;
use solana_client_helpers::{Client, ClientResult, SplToken};
use solana_sdk::{
    signature::{Keypair, Signer},
};


fn establish_connection(url: String, commit: CommitmentConfig) -> Client{
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
    let client = Client { client, payer };
    println!("Connection established {} {:?}\n",url,&client.get_version());
    client
}

fn establish_payer(client: &Client, payer:  &Pubkey) {

    let lamports = client.get_balance(payer).unwrap();

    println!("balane: {:?}\n",lamports);
    
    let sig = client.request_airdrop(payer, 5).unwrap();

    let s = client.confirm_transaction(&sig).unwrap();

    println!("{}\n",s);

    let lamports = client.get_balance(payer).unwrap();

    println!("{} {}\n",payer, lamports);


}

fn check_program(client: &Client, payer: &Pubkey, pro_pubkey: &Pubkey) -> Pubkey {
    let program_info = client.get_account(pro_pubkey).unwrap();
    if !program_info.executable {
        println!("not executable");
    }
    println!("Using program id: {}",pro_pubkey);

    let seed = "kaines";
    let greeted_pubkey = Pubkey::create_with_seed(payer, seed, pro_pubkey).unwrap();

    return greeted_pubkey;

}

fn say_hello(client: &Client ,greeted_pubkey: &Pubkey, pro_pubkey: &Pubkey, pay_pubkey: &Pubkey) {

    
    println!("saying hello to {}",greeted_pubkey);

    let key = AccountMeta::new(greeted_pubkey.clone(), false);
    let mut keys = Vec::new();
    keys.push(key);
    let data: [u8;0] = [];
    let mut ins_vec = vec![];
    let pro_pubke: Pubkey = Pubkey::from_str("HECBZ5UHKjzHL8NaNCz56EeGfTenX4GxZQgRTx8D6j7S").unwrap();
    let instruct = instruction::Instruction::new_with_bincode(pro_pubke, &data, keys);
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

}

use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct GreetingAccount {
    pub counter: u32
}

fn report_greeting(client: &RpcClient, greeted_pubkey: &Pubkey) {
    let account_info = client.get_account(greeted_pubkey).unwrap();
    let data = GreetingAccount::try_from_slice(&account_info.data).unwrap();
    println!("Greeted {} Times",data.counter);
}

fn main() {
    println!("Hello, world!");
    let url = "https://api.devnet.solana.com".to_string();
    let commit = CommitmentConfig::processed();

    let client = establish_connection(url.clone(), commit);

    let pay_pubkey: Pubkey = if let Ok(val) = Pubkey::from_str("6YX3wuJixYZ35xrN7wTbBiJVB8p4pPbVuCZjLUqhx2C3") {
        val
    } else {
        Pubkey::new_unique()
    };

    println!("{}",pay_pubkey);
    establish_payer(&client, &pay_pubkey);

    let pro_pubkey: Pubkey = Pubkey::from_str("HECBZ5UHKjzHL8NaNCz56EeGfTenX4GxZQgRTx8D6j7S").unwrap();

    let greeted_pubkey: Pubkey = check_program(&client,  &pay_pubkey, &pro_pubkey);

    say_hello(&client, &greeted_pubkey, &pro_pubkey, &pay_pubkey);
    report_greeting(&client, &greeted_pubkey);

}