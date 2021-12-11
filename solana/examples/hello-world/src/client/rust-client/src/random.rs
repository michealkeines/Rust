use clap::{App, Arg, ArgMatches};

use solana_remote_wallet::remote_wallet::RemoteWalletManager;

use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    message::Message,
    transaction::Transaction

};
use solana_program::{pubkey, system_instruction};
use solana_clap_utils::{
    input_validators::{is_valid_signer, is_url_or_moniker, normalize_to_url_if_moniker},
    input_parsers::pubkey_of_signer,
    keypair::{signer_from_path, CliSignerInfo},
    nonce::*,
    offline::{self, *},
    ArgConstant
};

use solana_client::{
    blockhash_query::BlockhashQuery,
    rpc_client::RpcClient
};

use std::{process::exit, sync::Arc};
use solana_cli_output::OutputFormat; 
mod config;
use config::Config;

const hello_world_program_id: &str = "7rB67P5ZpxhBe3zQx5znozDmhcXKPYyTtHRWvCrB7Xhd";


// TODO
// Need to use 
/*
 CreateAccountWithSeed {
        base: Pubkey,
        seed: String,
        lamports: u64,
        space: u64,
        owner: Pubkey,
    },

  To create the account data buffer for the helloworld smart contract

export type CreateAccountWithSeedParams = {
    /** The account that will transfer lamports to the created account */
    fromPubkey: PublicKey;
    /** Public key of the created account. Must be pre-calculated with PublicKey.createWithSeed() */
    newAccountPubkey: PublicKey;
    /** Base public key to use to derive the address of the created account. Must be the same as the base key used to create `newAccountPubkey` */
    basePubkey: PublicKey;
    /** Seed to use to derive the address of the created account. Must be the same as the seed used to create `newAccountPubkey` */
    seed: string;
    /** Amount of lamports to transfer to the created account */
    lamports: number;
    /** Amount of space in bytes to allocate to the created account */
    space: number;
    /** Public key of the program to assign as the owner of the created account */
    programId: PublicKey;
  };
 */

// Shit cribbed from solana_program_library/token/cli/
pub const MULTISIG_SIGNER_ARG: ArgConstant<'static> = ArgConstant {
    name: "multisig_signer",
    long: "multisig-signer",
    help: "Member signer of a multisig account",
};

type SignersOf = Vec<(Box<dyn Signer>, Pubkey)>;
pub fn signers_of(
    matches: &ArgMatches<'_>,
    name: &str,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> Result<Option<SignersOf>, Box<dyn std::error::Error>> {
    if let Some(values) = matches.values_of(name) {
        let mut results = Vec::new();
        for (i, value) in values.enumerate() {
            let name = format!("{}-{}", name, i + 1);
            let signer = signer_from_path(matches, value, &name, wallet_manager)?;
            let signer_pubkey = signer.pubkey();
            results.push((signer, signer_pubkey));
        }
        Ok(Some(results))
    } else {
        Ok(None)
    }
}

fn get_config<'a>(
    matches: &ArgMatches<'_>,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> Config {
    let config = {
        let mut bulk_signers: Vec<Box<dyn Signer>> = Vec::new();

        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            println!("loading config file: {:?}", config_file);
            solana_cli_config::Config::load(config_file).unwrap()
        } else {
            let default_config = solana_cli_config::Config::default();
            println!("loading default: {:?}", default_config);
            default_config
        };
        /*
        let json_rpc_url = normalize_to_url_if_moniker(
            matches
                .value_of("json_rpc_url")
                .unwrap_or(&cli_config.json_rpc_url),
        );
        */
        let json_rpc_url = cli_config.json_rpc_url;
        println!("json_rpc_url: {:?}", json_rpc_url);
        let websocket_url = solana_cli_config::Config::compute_websocket_url(&json_rpc_url);

        let (signer, fee_payer) = signer_from_path(
            matches,
            matches
                .value_of("fee_payer")
                .unwrap_or(&cli_config.keypair_path),
            "fee_payer",
            wallet_manager,
        )
        .map(|s| {
            let p = s.pubkey();
            (s, p)
        })
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            exit(1);
        });
        bulk_signers.push(signer);

        let verbose = matches.is_present("verbose");
        let output_format = matches
            .value_of("output_format")
            .map(|value| match value {
                "json" => OutputFormat::Json,
                "json-compact" => OutputFormat::JsonCompact,
                _ => unreachable!(),
            })
            .unwrap_or(if verbose {
                OutputFormat::DisplayVerbose
            } else {
                OutputFormat::Display
            });

        let nonce_account = pubkey_of_signer(matches, NONCE_ARG.name, wallet_manager)
            .unwrap_or_else(|e| {
                eprintln!("error: {}", e);
                exit(1);
            });
        let nonce_authority = if nonce_account.is_some() {
            let (signer, nonce_authority) = signer_from_path(
                matches,
                matches
                    .value_of(NONCE_AUTHORITY_ARG.name)
                    .unwrap_or(&cli_config.keypair_path),
                NONCE_AUTHORITY_ARG.name,
                wallet_manager,
            )
            .map(|s| {
                let p = s.pubkey();
                (s, p)
            })
            .unwrap_or_else(|e| {
                eprintln!("error: {}", e);
                exit(1);
            });
            bulk_signers.push(signer);

            Some(nonce_authority)
        } else {
            None
        };

        let blockhash_query = BlockhashQuery::new_from_matches(matches);
        let sign_only = matches.is_present(SIGN_ONLY_ARG.name);
        let dump_transaction_message = matches.is_present(DUMP_TRANSACTION_MESSAGE.name);

        let multisig_signers = signers_of(matches, MULTISIG_SIGNER_ARG.name, wallet_manager)
            .unwrap_or_else(|e| {
                eprintln!("error: {}", e);
                exit(1);
            });

        Config {
            rpc_client: Arc::new(RpcClient::new_with_commitment(
                json_rpc_url,
                CommitmentConfig::confirmed(),
            )),
            websocket_url,
            output_format,
            fee_payer,
            default_keypair_path: cli_config.keypair_path,
            nonce_account,
            nonce_authority,
            blockhash_query,
            sign_only,
            dump_transaction_message,
        }
    };
    config
}

fn get_signer(
    // Something from ArgMatches
    matches: &ArgMatches<'_>,
    // What is this?
    keypair_name: &str,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> Option<(Box<dyn Signer>, Pubkey)> {
    matches.value_of(keypair_name).map(|path| {
        let signer =
            signer_from_path(matches, path, keypair_name, wallet_manager).unwrap_or_else(|e| {
                eprintln!("error: {}", e);
                exit(1);
            });
        let signer_pubkey = signer.pubkey();
        (signer, signer_pubkey)
    })
}
pub fn hello_instruction(
    program_id: &Pubkey,
    authority: &Pubkey,
    lamports: u64 
) -> Result<Vec<Instruction>, ProgramError> {
    let pda = Pubkey::create_with_seed(program_id, "seed", program_id).unwrap();
    let account_meta = vec![AccountMeta::new(pda, false)];
    println!("pda: {:?}", pda);
    let create_account_instruction = system_instruction::create_account_with_seed(authority, &pda, program_id, "seed", lamports, 4, program_id);
    let instruction = Instruction::new_with_bytes(*program_id, &Vec::<u8>::new(), account_meta);
    println!("hello_instruction: {:?}", create_account_instruction);
    Ok(vec![create_account_instruction,instruction])
   // Ok(vec![create_account_instruction])
}

/// end shit cribbed from token/cli

fn main() {

    let mut wallet_manager = None;
    let matches = App::new("invoke helloworld in solana")
        .version("0.1.0")
        .author("fhools")
        .arg(
            Arg::with_name("keypair")
                .long("keypair")
                .value_name("KEYPAIR")
                .validator(is_valid_signer)
                .help("specify keypair")
                .required(true),
        )
        .arg(
            Arg::with_name("program-keypair")
                .long("program-keypair")
                .value_name("PROGRAM_KEYPAIR")
                .validator(is_valid_signer)
                .help("specify program keypair")
                .required(true),
        )
        .arg({
            let arg = Arg::with_name("config_file")
                .short("C")
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("specify solana cli config");
            if let Some(ref config_file) = *solana_cli_config::CONFIG_FILE {
                arg.default_value(config_file)
            } else {
                arg
            }
            }
        )
        .get_matches();
    let signer = get_signer(&matches, "keypair", &mut wallet_manager);
    if signer.is_some() {
        println!("Got valid signer {:?}", signer);
    } else {
        println!("Could not get valid signer");
    }
    let program_signer = get_signer(&matches, "program-keypair", &mut wallet_manager);
    if program_signer.is_some() {
        println!("Got program signer: {:?}", program_signer);
    } else {
        println!("Could not get program signer");
    }

    let config = get_config(&matches, &mut wallet_manager);
    let address = config.default_address(&matches, &mut wallet_manager).unwrap();
    println!("Address is: {:?}", address);

    let hello_program_id = pubkey!("7rB67P5ZpxhBe3zQx5znozDmhcXKPYyTtHRWvCrB7Xhd"); 
    let instructions = hello_instruction(&hello_program_id, &address, 100).unwrap();
    println!("instructions: {:?}", instructions);
    let mut bulk_signers : Vec::<Box<dyn Signer>> = Vec::new();

    bulk_signers.push(signer.unwrap().0);
    bulk_signers.push(program_signer.unwrap().0);

    let message = Message::new(&instructions, Some(&config.fee_payer));
    
    let signer_info = CliSignerInfo {
        signers: bulk_signers
    };
    let signers = signer_info.signers_for_message(&message);

    let mut transaction = Transaction::new_unsigned(message);

    let (recent_blockhash, _fee_calculator) = config.blockhash_query
        .get_blockhash_and_fee_calculator(&config.rpc_client, config.rpc_client.commitment())
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            exit(1);
        });
    transaction.try_sign(&signers, recent_blockhash).unwrap();
    println!("transaction: {:?}", transaction);
    config.rpc_client.send_and_confirm_transaction_with_spinner(&transaction).unwrap();

}
