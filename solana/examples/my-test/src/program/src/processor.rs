
use borsh::{BorshDeserialize, BorshSerialize};
//use metaplex_token_vault::{error::VaultError, state::VaultState};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{
     instruction::{MetadataInstruction},
     state::{ Key, MasterEditionV2, Data, Creator, PREFIX, EDITION, MAX_MASTER_EDITION_LEN},
     utils::{
         CreateMetadataAccountsLogicArgs, process_create_metadata_accounts_logic,
         MintNewEditionFromMasterEditionViaTokenLogicArgs, process_mint_new_edition_from_master_edition_via_token_logic
    },
};
use spl_token::state::{Account, Mint};

use crate::{
    utils::{
        assert_owned_by,
        assert_update_authority_is_correct,
        assert_data_valid,
        puff_out_data_fields,
        assert_initialized,
        assert_derivation,
        assert_token_program_matches_package,
        assert_mint_authority_matches_mint,
        create_or_allocate_account_raw,
        transfer_mint_authority,
    },
    state::Metadata,
    error::MetadataError
};



pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &'a [u8]
) -> ProgramResult {
    msg!("inside processinstruction");
    
    let instruction = MetadataInstruction::try_from_slice(input)?;

    msg!("Deserialized input check passed.");

    match instruction {
        MetadataInstruction::CreateMetadataAccount(args) => {
            msg!("Inside Create Metadata Account match");
            process_create_metadata_accounts(program_id, accounts, args.data, args.is_mutable)
            
        },
        MetadataInstruction::UpdateMetadataAccount(args) => {
            msg!("Inside Update Metadata Account match");
            process_update_metadata_accounts(program_id, accounts, args.data, args.update_authority, args.primary_sale_happened)
        },   
        MetadataInstruction::CreateMasterEdition(args) => {
            msg!("Inside Create Master Edition match");
            process_create_master_edition(program_id, accounts, args.max_supply)
        },
        MetadataInstruction::MintNewEditionFromMasterEditionViaToken(args) => {
            msg!("Inside Mint new Edition from Master edition using Token account match");
            process_mint_new_edition_from_master_edition_via_token(program_id, accounts, args.edition, false)
        }
    }
}

fn process_create_metadata_accounts<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: Data,
    is_mutable: bool
) -> ProgramResult { 
    msg!("Inside process create metadata accounts Function");

    let account_info_iter = &mut accounts.iter();
    let metadata_account_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let mint_authority_info = next_account_info(account_info_iter)?;
    let payer_account_info = next_account_info(account_info_iter)?;
    let update_authority_info = next_account_info(account_info_iter)?;
    let system_account_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    let allow_direct_creator_writes = false;

    process_create_metadata_accounts_logic(
        &program_id,
        CreateMetadataAccountsLogicArgs {
            metadata_account_info,
            mint_info,
            mint_authority_info,
            payer_account_info,
            update_authority_info,
            system_account_info,
            rent_info,
        },
        data,
        allow_direct_creator_writes,
        is_mutable,
    )

    //Ok(())

}


fn process_update_metadata_accounts<'a>(
    program_id: &'a Pubkey,
    accounts: &'a[AccountInfo<'a>], 
    data: Option<Data>,
    update_authority: Option<Pubkey>,
    primary_sale_happened: Option<bool>
) -> ProgramResult {
    msg!("Inside process update metadata accounts Function");
    let account_info_iter = &mut accounts.iter();

    let metadata_account_info = next_account_info(account_info_iter)?;
    let update_authority_info = next_account_info(account_info_iter)?;

    // we borrow the account data for metadata account
    let mut metadata = Metadata::from_account_info(metadata_account_info)?;

    assert_owned_by(metadata_account_info, program_id)?;
    assert_update_authority_is_correct(&metadata, update_authority_info)?;

    if let Some(optional_data) = data {
        if metadata.is_mutable {
            assert_data_valid(
                &optional_data,
                update_authority_info.key,
                &metadata,
                false,
                update_authority_info.is_signer,
                true,
            )?;
            metadata.data = optional_data;
        } else {
            return Err(MetadataError::DataIsImmutable.into());
        }
    }
    msg!("input data is valid");
    if let Some(val) = update_authority {
        metadata.update_authority = val;
    }

    if let Some(val) = primary_sale_happened {
        if val {
            metadata.primary_sale_happened = val
        } else {
            return Err(MetadataError::PrimarySaleCanOnlyBeFlippedToTrue.into());
        }
    }
    msg!("is primary sale chek");
    puff_out_data_fields(&mut metadata);
    msg!("puffed feilds");

    // we write the modified data back to the account
    metadata.serialize(&mut *metadata_account_info.data.borrow_mut())?;
    Ok(())
}

pub fn process_create_master_edition(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    max_supply: Option<u64>,
) -> ProgramResult {
    msg!("Inside process create master edition function");

    let account_info_iter = &mut accounts.iter();

    let edition_account_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let update_authority_info = next_account_info(account_info_iter)?;
    let mint_authority_info = next_account_info(account_info_iter)?;
    let payer_account_info = next_account_info(account_info_iter)?;
    let metadata_account_info = next_account_info(account_info_iter)?;
    // have to create this
    let token_program_info = next_account_info(account_info_iter)?;
    let system_account_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    let metadata = Metadata::from_account_info(metadata_account_info)?;
    let mint: Mint = assert_initialized(mint_info)?;

    let bump_seed = assert_derivation(
        program_id,
        edition_account_info,
        &[
            PREFIX.as_bytes(),
            program_id.as_ref(),
            &mint_info.key.as_ref(),
            EDITION.as_bytes(),
        ],
    )?;

    assert_token_program_matches_package(token_program_info)?;
    assert_mint_authority_matches_mint(&mint.mint_authority, mint_authority_info)?;
    assert_owned_by(metadata_account_info, program_id)?;
    assert_owned_by(mint_info, &spl_token::id())?;

    if metadata.mint != *mint_info.key {
        return Err(MetadataError::MintMismatch.into());
    }

    if mint.decimals != 0 {
        return Err(MetadataError::EditionMintDecimalsShouldBeZero.into());
    }

    assert_update_authority_is_correct(&metadata, update_authority_info)?;

    if mint.supply != 1 {
        return Err(MetadataError::EditionsMustHaveExactlyOneToken.into());
    }

    let edition_authority_seeds = &[
        PREFIX.as_bytes(),
        program_id.as_ref(),
        &mint_info.key.as_ref(),
        EDITION.as_bytes(),
        &[bump_seed],
    ];

    create_or_allocate_account_raw(
        *program_id,
        edition_account_info,
        rent_info,
        system_account_info,
        payer_account_info,
        MAX_MASTER_EDITION_LEN,
        edition_authority_seeds,
    )?;

    let mut edition = MasterEditionV2::from_account_info(edition_account_info)?;

    edition.key = Key::MasterEditionV2;
    edition.supply = 0;
    edition.max_supply = max_supply;
    edition.serialize(&mut *edition_account_info.data.borrow_mut())?;

    // While you can't mint any more of your master record, you can
    // mint as many limited editions as you like within your max supply.
    transfer_mint_authority(
        edition_account_info.key,
        edition_account_info,
        mint_info,
        mint_authority_info,
        token_program_info,
    )?;

    Ok(())
}

pub fn process_mint_new_edition_from_master_edition_via_token<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    edition: u64,
    ignore_owner_signer: bool,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let new_metadata_account_info = next_account_info(account_info_iter)?;
    let new_edition_account_info = next_account_info(account_info_iter)?;
    let master_edition_account_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let edition_marker_info = next_account_info(account_info_iter)?;
    let mint_authority_info = next_account_info(account_info_iter)?;
    let payer_account_info = next_account_info(account_info_iter)?;
    let owner_account_info = next_account_info(account_info_iter)?;
    let token_account_info = next_account_info(account_info_iter)?;
    let update_authority_info = next_account_info(account_info_iter)?;
    let master_metadata_account_info = next_account_info(account_info_iter)?;
    let token_program_account_info = next_account_info(account_info_iter)?;
    let system_account_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    process_mint_new_edition_from_master_edition_via_token_logic(
        &program_id,
        MintNewEditionFromMasterEditionViaTokenLogicArgs {
            new_metadata_account_info,
            new_edition_account_info,
            master_edition_account_info,
            mint_info,
            edition_marker_info,
            mint_authority_info,
            payer_account_info,
            owner_account_info,
            token_account_info,
            update_authority_info,
            master_metadata_account_info,
            token_program_account_info,
            system_account_info,
            rent_info,
        },
        edition,
        ignore_owner_signer,
    )
}
