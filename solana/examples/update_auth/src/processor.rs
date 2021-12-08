use crate::{
    error::MetadataError,
    instruction::MetadataInstruction,
    state::{
        Data, Key, MasterEditionV1, MasterEditionV2, Metadata, EDITION, MAX_MASTER_EDITION_LEN,
        PREFIX,
    },
    utils::{
        assert_data_valid, assert_derivation, assert_initialized,
        assert_mint_authority_matches_mint, assert_owned_by, assert_signer,
        assert_token_program_matches_package, assert_update_authority_is_correct,
        create_or_allocate_account_raw, get_owner_from_token_account,
        process_create_metadata_accounts_logic,
        process_mint_new_edition_from_master_edition_via_token_logic, puff_out_data_fields,
        transfer_mint_authority, CreateMetadataAccountsLogicArgs,
        MintNewEditionFromMasterEditionViaTokenLogicArgs,
    },
};
use arrayref::array_ref;
use borsh::{BorshDeserialize, BorshSerialize};
//use metaplex_token_vault::{error::VaultError, state::VaultState};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::state::{Account, Mint};

// this function is called inside the entrypoint.rs
pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &[u8],
) -> ProgramResult {
    // deserialize the input
    let instruction = MetadataInstruction::try_from_slice(input)?;
    // find which instruction 
    match instruction {
        // this is updatemetadataacout impl
        MetadataInstruction::UpdateMetadataAccount(args) => {
            msg!("Instruction: Update Metadata Accounts");
            process_update_metadata_accounts(
                program_id,
                accounts,
                args.data,
                args.update_authority,
                args.primary_sale_happened,
            )
        }
    }
}

pub fn process_update_metadata_accounts(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    optional_data: Option<Data>,
    update_authority: Option<Pubkey>,
    primary_sale_happened: Option<bool>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter(); // iterate over all input accounts

    let metadata_account_info = next_account_info(account_info_iter)?; // first account
    let update_authority_info = next_account_info(account_info_iter)?; // second account
    let mut metadata = Metadata::from_account_info(metadata_account_info)?; // get the current metadata

    // check if program id == first account id
    assert_owned_by(metadata_account_info, program_id)?;

    // check if current update authority is same in the retrived metadata
    assert_update_authority_is_correct(&metadata, update_authority_info)?;

    if let Some(data) = optional_data {
        // if metadata is mutable and data passed is in valid format
        if metadata.is_mutable {
            assert_data_valid(
                &data,
                update_authority_info.key,
                &metadata,
                false,
                update_authority_info.is_signer,
                true,
            )?;
            // update the metadata
            metadata.data = data;
        } else {
            return Err(MetadataError::DataIsImmutable.into());
        }
    }

    // update the update_authority
    if let Some(val) = update_authority {
        metadata.update_authority = val;
    }

    // set if sale possible to true
    if let Some(val) = primary_sale_happened {
        if val {
            metadata.primary_sale_happened = val
        } else {
            return Err(MetadataError::PrimarySaleCanOnlyBeFlippedToTrue.into());
        }
    }
    // not sure what this does, i guess some sort of padding?
    puff_out_data_fields(&mut metadata);

    
    metadata.serialize(&mut *metadata_account_info.data.borrow_mut())?;
    Ok(())
}
