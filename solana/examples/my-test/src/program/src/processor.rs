
use borsh::{BorshDeserialize, BorshSerialize};
//use metaplex_token_vault::{error::VaultError, state::VaultState};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{
     instruction::MetadataInstruction,
     state::{Data, Creator},
     utils::{
         CreateMetadataAccountsLogicArgs, process_create_metadata_accounts_logic
     },
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
        MetadataInstruction::Test1(args) => {
            msg!("Inside test1 match");
            process_test1(program_id, accounts, args.data, args.is_mutable)
            
        },
        MetadataInstruction::Test2(args) => {
            msg!("Inside test2 match");
            process_test2(program_id, accounts, args.data, args.update_authority, args.primary_sale_happened)
        },   
    }
}

fn process_test1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: Data,
    is_mutable: bool
) -> ProgramResult { 
    msg!("Inside Process 1 Function");

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
use crate::{
    utils::{assert_owned_by, assert_update_authority_is_correct, assert_data_valid, puff_out_data_fields},
    state::Metadata,
    error::MetadataError
};

fn process_test2<'a>(
    program_id: &'a Pubkey,
    accounts: &'a[AccountInfo<'a>], 
    data: Option<Data>,
    update_authority: Option<Pubkey>,
    primary_sale_happened: Option<bool>
) -> ProgramResult {
    msg!("Inside Process 2 Function");
    let account_info_iter = &mut accounts.iter();

    let metadata_account_info = next_account_info(account_info_iter)?;
    let update_authority_info = next_account_info(account_info_iter)?;
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
    metadata.serialize(&mut *metadata_account_info.data.borrow_mut())?;
    Ok(())
}

