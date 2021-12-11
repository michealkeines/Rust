


// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
// use std::time::Duration;
// use solana_sdk::{
//     client::SyncClient,
//     signature,
//     transaction::Transaction,
//     commitment_config::CommitmentConfig, 
//     signature::{Keypair, Signer},
//     signature::read_keypair_file
// };
// use solana_program::{
//     instruction::AccountMeta, 
//     instruction::Instruction,
//     pubkey::Pubkey
// };
// use solana_client::{rpc_client::RpcClient,thin_client};
// use std::io::Result;



// // pub fn establish_connection() -> Result<RpcClient> {
// //     let rpc_addr = "127.0.0.1:8899";
// //     let timeout = 1000;

// //     println!("Connectin to solana node, RPC: {}, timeout: {}ms", rpc_addr, timeout);

// //     let rpc_addr: SocketAddr = rpc_addr.parse().expect("");

// //     let client = RpcClient::new_socket_with_timeout(rpc_addr, Duration::from_millis(timeout));

// //     let version = client.get_version().unwrap();

// //     println!("RPC version: {:?}", version);

// //     Ok(client)
// // }

// pub struct Config {
//     json_rpc_url: String,
//     keypair: Keypair
// }

// pub fn load_config() -> Config {
//     let config_file = solana_cli_config::CONFIG_FILE.as_ref().ok_or_else(|| println!("config file path")).unwrap();
//     let cli_config = solana_cli_config::Config::load(&config_file).unwrap();
//     let json_rpc_url = cli_config.json_rpc_url;
//     let keypair = read_keypair_file(&cli_config.keypair_path).map_err(|e| println!("{}", e)).unwrap();
//     Config {
//         json_rpc_url,
//         keypair
//     }
// }

// pub fn connect(config: &Config) -> Result<RpcClient> {

//     println!("connecting to solana node at {}", config.json_rpc_url);
//     let client = RpcClient::new_with_commitment(config.json_rpc_url.clone(), CommitmentConfig::confirmed());

//     let version = client.get_version().unwrap();
//     println!("RPC version: {:?}", version);

//     Ok(client)
// }

// static DEPLOY_PATH: &str = "/home/micheal/Rust/solana/examples/hello-world/src/program/target/deploy";
// static PROGRAM_SO_PATH: &str = "solana_bpf_helloworld.so";
// static PROGRAM_KEYPAIR_PATH: &str = "solana_bpf_helloworld-keypair.json";

// pub fn get_program_keypair(client: &RpcClient) -> Result<Keypair> {
//     let deploy_path = format!("{}", DEPLOY_PATH);
//     let program_so_path = format!("{}/{}", deploy_path, PROGRAM_SO_PATH);
//     let program_keypair_path = format!("{}/{}", deploy_path, PROGRAM_KEYPAIR_PATH);

//     println!("loading program keypair from {}", program_keypair_path);

//     let program_keypair = read_keypair_file(&program_keypair_path)
//         .map_err(|e| println!("{}", e)).unwrap();

//     let program_id = program_keypair.pubkey();

//     println!("program id: {}", program_id);

//     let account = client.get_account(&program_id).unwrap();

//     if !account.executable {
//         println!("solana account not executable");
//     }

//     Ok(program_keypair)
// }

// fn create_instruction(program_id: &Pubkey, program_instance: &Pubkey) -> Result<Instruction> {
//     let data = vec![];
//     Ok(Instruction {
//         program_id: *program_id,
//         accounts: vec![
//         AccountMeta::new(*program_instance, true),
//         AccountMeta::new(*program_instance, false)
//         ],
//         data
//     })
// }

// pub fn upload_plant(
//     config: &Config,
//     client: &RpcClient,
//     program: &Keypair, // config keypair
//     program_account: &Pubkey // greeted pair
// ) -> Result<()> {
//     let inst = create_instruction(&program.pubkey(), program_account).unwrap();
//     println!("\n\n{:?}\n\n",inst);
//     let mut tx = Transaction::new_with_payer(
//     &[inst], Some(&config.keypair.pubkey()));
   
//     let blockhash = client.get_recent_blockhash().unwrap().0;
//     println!("{}",blockhash);
//     tx.try_sign(&[&config.keypair, program], blockhash).unwrap();
//     client.send_and_confirm_transaction(&tx).unwrap();

//     Ok(())
// }




// fn main() {
//     let config = load_config();
//     let client = connect(&config).unwrap();
//     let pro_pair = get_program_keypair(&client).unwrap();
//     let instruct = create_instruction(&config.keypair.pubkey(), &pro_pair.pubkey()).unwrap();
//     let seed = "kaines";
//     let greeted_pubkey = Pubkey::create_with_seed(&config.keypair.pubkey(), seed, &pro_pair.pubkey()).unwrap();
//     println!("{:?}",greeted_pubkey);
//     upload_plant(&config, &client, &pro_pair, &greeted_pubkey);

// }