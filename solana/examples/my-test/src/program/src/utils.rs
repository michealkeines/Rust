use crate::{
    error::MetadataError,
    state::{
        get_reservation_list, Data, EditionMarker, Key, MasterEditionV1, Metadata, EDITION,
        EDITION_MARKER_BIT_SIZE, MAX_CREATOR_LIMIT, MAX_EDITION_LEN, MAX_EDITION_MARKER_SIZE,
        MAX_MASTER_EDITION_LEN, MAX_METADATA_LEN, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH,
        MAX_URI_LENGTH, PREFIX,
    },
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_option::COption,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::{
    instruction::{set_authority, AuthorityType},
    state::{Account, Mint},
};
use std::convert::TryInto;


/// TokenMintToParams
pub struct TokenMintToParams<'a: 'b, 'b> {
    /// mint
    pub mint: AccountInfo<'a>,
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: Option<&'b [&'b [u8]]>,
    /// token_program
    pub token_program: AccountInfo<'a>,
}

pub fn assert_derivation(
    program_id: &Pubkey,
    account: &AccountInfo,
    path: &[&[u8]],
) -> Result<u8, ProgramError> {
    let (key, bump) = Pubkey::find_program_address(&path, program_id);
    if key != *account.key {
        return Err(MetadataError::DerivedKeyInvalid.into());
    }
    Ok(bump)
}

pub fn assert_signer(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_signer {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        Err(MetadataError::IncorrectOwner.into())
    } else {
        Ok(())
    }
}

pub fn assert_token_program_matches_package(token_program_info: &AccountInfo) -> ProgramResult {
    if *token_program_info.key != spl_token::id() {
        return Err(MetadataError::InvalidTokenProgram.into());
    }

    Ok(())
}

pub fn assert_update_authority_is_correct(
    metadata: &Metadata,
    update_authority_info: &AccountInfo,
) -> ProgramResult {
    if metadata.update_authority != *update_authority_info.key {
        return Err(MetadataError::UpdateAuthorityIncorrect.into());
    }

    if !update_authority_info.is_signer {
        return Err(MetadataError::UpdateAuthorityIsNotSigner.into());
    }

    Ok(())
}

pub fn try_from_slice_checked<T: BorshDeserialize>(
    data: &[u8],
    data_type: Key,
    data_size: usize,
) -> Result<T, ProgramError> {
    if (data[0] != data_type as u8 && data[0] != Key::Uninitialized as u8)
        || data.len() != data_size
    {
        return Err(MetadataError::DataTypeMismatch.into());
    }

    let result: T = try_from_slice_unchecked(data)?;

    Ok(result)
}

pub struct CreateMetadataAccountsLogicArgs<'a> {
    pub metadata_account_info: &'a AccountInfo<'a>,
    pub mint_info: &'a AccountInfo<'a>,
    pub mint_authority_info: &'a AccountInfo<'a>,
    pub payer_account_info: &'a AccountInfo<'a>,
    pub update_authority_info: &'a AccountInfo<'a>,
    pub system_account_info: &'a AccountInfo<'a>,
    pub rent_info: &'a AccountInfo<'a>,
}

// This equals the program address of the metadata program:
// AqH29mZfQFgRpfwaPoTMWSKJ5kqauoc1FwVBRksZyQrt
// IMPORTANT NOTE
// This allows the upgrade authority of the Token Metadata program to create metadata for SPL tokens.
// This only allows the upgrade authority to do create general metadata for the SPL token, it does not
// allow the upgrade authority to add or change creators.
const SEED_AUTHORITY: Pubkey = Pubkey::new_from_array([
    0x92, 0x17, 0x2c, 0xc4, 0x72, 0x5d, 0xc0, 0x41, 0xf9, 0xdd, 0x8c, 0x51, 0x52, 0x60, 0x04, 0x26,
    0x00, 0x93, 0xa3, 0x0b, 0x02, 0x73, 0xdc, 0xfa, 0x74, 0x92, 0x17, 0xfc, 0x94, 0xa2, 0x40, 0x49,
]);
pub fn assert_mint_authority_matches_mint(
    mint_authority: &COption<Pubkey>,
    mint_authority_info: &AccountInfo,
) -> ProgramResult {
    match mint_authority {
        COption::None => {
            return Err(MetadataError::InvalidMintAuthority.into());
        }
        COption::Some(key) => {
            if mint_authority_info.key != key {
                return Err(MetadataError::InvalidMintAuthority.into());
            }
        }
    }

    if !mint_authority_info.is_signer {
        return Err(MetadataError::NotMintAuthority.into());
    }

    Ok(())
}

pub fn get_mint_authority(account_info: &AccountInfo) -> Result<COption<Pubkey>, ProgramError> {
    // In token program, 36, 8, 1, 1 is the layout, where the first 36 is mint_authority
    // so we start at 0.
    let data = account_info.try_borrow_data().unwrap();
    let authority_bytes = array_ref![data, 0, 36];

    Ok(unpack_coption_key(&authority_bytes)?)
}

/// Unpacks COption from a slice, taken from token program
fn unpack_coption_key(src: &[u8; 36]) -> Result<COption<Pubkey>, ProgramError> {
    let (tag, body) = array_refs![src, 4, 32];
    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(*body))),
        _ => Err(ProgramError::InvalidAccountData),
    }
}


/// Create a new account instruction
pub fn process_create_metadata_accounts_logic(
    program_id: &Pubkey,
    accounts: CreateMetadataAccountsLogicArgs,
    data: Data,
    allow_direct_creator_writes: bool,
    mut is_mutable: bool,
) -> ProgramResult {
    let CreateMetadataAccountsLogicArgs {
        metadata_account_info,
        mint_info,
        mint_authority_info,
        payer_account_info,
        update_authority_info,
        system_account_info,
        rent_info,
    } = accounts;
    msg!("inside create metadata account logic");
    let mut update_authority_key = *update_authority_info.key;
    let existing_mint_authority = get_mint_authority(mint_info)?;
    msg!("got mint authority");
    // IMPORTANT NOTE
    // This allows the Metaplex Foundation to Create but not update metadata for SPL tokens that have not populated their metadata.
    assert_mint_authority_matches_mint(&existing_mint_authority, mint_authority_info).or_else(
        |e| {
            // Allow seeding by the authority seed populator
            if mint_authority_info.key == &SEED_AUTHORITY && mint_authority_info.is_signer {
                // When metadata is seeded, the mint authority should be able to change it
                if let COption::Some(auth) = existing_mint_authority {
                    update_authority_key = auth;
                    is_mutable = true;
                }
                Ok(())
            } else {
                Err(e)
            }
        },
    )?;
    msg!("assert mint auth matches mint");
    assert_owned_by(mint_info, &spl_token::id())?;
    msg!("asset owned by");
    let metadata_seeds = &[
        PREFIX.as_bytes(),
        program_id.as_ref(),
        mint_info.key.as_ref(),
    ];
    let (metadata_key, metadata_bump_seed) =
        Pubkey::find_program_address(metadata_seeds, program_id);
    let metadata_authority_signer_seeds = &[
        PREFIX.as_bytes(),
        program_id.as_ref(),
        mint_info.key.as_ref(),
        &[metadata_bump_seed],
    ];

    if metadata_account_info.key != &metadata_key {
        return Err(MetadataError::InvalidMetadataKey.into());
    }
    msg!("all set for creating account igues");

    create_or_allocate_account_raw(
        *program_id,
        metadata_account_info,
        rent_info,
        system_account_info,
        payer_account_info,
        MAX_METADATA_LEN,
        metadata_authority_signer_seeds,
    )?;

    msg!("created account");

    let mut metadata = Metadata::from_account_info(metadata_account_info)?;
    msg!("got metadata account info");
    assert_data_valid(
        &data,
        &update_authority_key,
        &metadata,
        allow_direct_creator_writes,
        update_authority_info.is_signer,
        false,
    )?;
    msg!("assert created account");
    metadata.mint = *mint_info.key;
    metadata.key = Key::MetadataV1;
    metadata.data = data;
    metadata.is_mutable = is_mutable;
    metadata.update_authority = update_authority_key;

    puff_out_data_fields(&mut metadata);
    msg!("puffed data fields");

    let edition_seeds = &[
        PREFIX.as_bytes(),
        program_id.as_ref(),
        metadata.mint.as_ref(),
        EDITION.as_bytes(),
    ];
    let (_, edition_bump_seed) = Pubkey::find_program_address(edition_seeds, program_id);
    metadata.edition_nonce = Some(edition_bump_seed);
    msg!("found a program address");

    metadata.serialize(&mut *metadata_account_info.data.borrow_mut())?;

    msg!("serialized alla data");

    Ok(())
}

pub fn assert_data_valid(
    data: &Data,
    update_authority: &Pubkey,
    existing_metadata: &Metadata,
    allow_direct_creator_writes: bool,
    update_authority_is_signer: bool,
    is_updating: bool,
) -> ProgramResult {
    msg!("insside assert data vaild for metadata account");
    if data.name.len() > MAX_NAME_LENGTH {
        return Err(MetadataError::NameTooLong.into());
    }
    msg!("name lenght passed");
    if data.symbol.len() > MAX_SYMBOL_LENGTH {
        return Err(MetadataError::SymbolTooLong.into());
    }
    msg!("symbol length passed");
    if data.uri.len() > MAX_URI_LENGTH {
        return Err(MetadataError::UriTooLong.into());
    }
    msg!("uril length passed");
    if data.seller_fee_basis_points > 10000 {
        return Err(MetadataError::InvalidBasisPoints.into());
    }
    msg!("selller base points passed");
    if data.creators.is_some() {
        if let Some(creators) = &data.creators {
            if creators.len() > MAX_CREATOR_LIMIT {
                return Err(MetadataError::CreatorsTooLong.into());
            }
            msg!("created limit passed");
            if creators.is_empty() {
                return Err(MetadataError::CreatorsMustBeAtleastOne.into());
            } else {
                msg!("creators array not empty");
                let mut found = false;
                let mut total: u8 = 0;
                for i in 0..creators.len() {
                    let creator = &creators[i];
                    for j in (i + 1)..creators.len() {
                        if creators[j].address == creator.address {
                            return Err(MetadataError::DuplicateCreatorAddress.into());
                        }
                    }
                    msg!("create address not deuplicate");
                    total = total
                        .checked_add(creator.share)
                        .ok_or(MetadataError::NumericalOverflowError)?;

                    if creator.address == *update_authority {
                        found = true;
                    }
                    msg!("found: {}",found);
                    // Dont allow metadata owner to unilaterally say a creator verified...
                    // cross check with array, only let them say verified=true here if
                    // it already was true and in the array.
                    // Conversely, dont let a verified creator be wiped.
                    if (!update_authority_is_signer || creator.address != *update_authority)
                        && !allow_direct_creator_writes
                    {
                        msg!("inside the metadata owner verify write access check");
                        if let Some(existing_creators) = &existing_metadata.data.creators {
                            match existing_creators
                                .iter()
                                .find(|c| c.address == creator.address)
                            {
                                Some(existing_creator) => {
                                    msg!("Some {:?}",existing_creator);
                                    if creator.verified && !existing_creator.verified {
                                        return Err(
                                            MetadataError::CannotVerifyAnotherCreator.into()
                                        );
                                    } else if !creator.verified && existing_creator.verified {
                                        return Err(
                                            MetadataError::CannotUnverifyAnotherCreator.into()
                                        );
                                    }
                                }
                                None => {
                                    msg!("None {:?}",creator);
                                    if creator.verified {
                                        return Err(
                                            MetadataError::CannotVerifyAnotherCreator.into()
                                        );
                                    }
                                }
                            }
                            msg!("passed exisisting metadata somtigijidjid");
                        } else {
                            msg!("{:?}",creator);
                            if creator.verified {
                                return Err(MetadataError::CannotVerifyAnotherCreator.into());
                            }
                            msg!("creator verified");
                        }
                    }
                }
                msg!("found allow, is updating check");
                if !found && !allow_direct_creator_writes && !is_updating {
                    return Err(MetadataError::MustBeOneOfCreators.into());
                }
                msg!("allow direct access to creteor wirtes");
                if total != 100 {
                    return Err(MetadataError::ShareTotalMustBe100.into());
                }
                msg!("somehingis not equal to 100");
            }
        }
    }

    Ok(())
}


#[inline(always)]
pub fn create_or_allocate_account_raw<'a>(
    program_id: Pubkey,
    new_account_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    let rent = &Rent::from_account_info(rent_sysvar_info)?;
    let required_lamports = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    if required_lamports > 0 {
        msg!("Transfer {} lamports to the new account", required_lamports);
        invoke(
            &system_instruction::transfer(&payer_info.key, new_account_info.key, required_lamports),
            &[
                payer_info.clone(),
                new_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    let accounts = &[new_account_info.clone(), system_program_info.clone()];

    msg!("Allocate space for the account");
    invoke_signed(
        &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
        accounts,
        &[&signer_seeds],
    )?;

    msg!("Assign the account to the owning program");
    invoke_signed(
        &system_instruction::assign(new_account_info.key, &program_id),
        accounts,
        &[&signer_seeds],
    )?;

    Ok(())
}

/// Strings need to be appended with `\0`s in order to have a deterministic length.
/// This supports the `memcmp` filter  on get program account calls.
/// NOTE: it is assumed that the metadata fields are never larger than the respective MAX_LENGTH
pub fn puff_out_data_fields(metadata: &mut Metadata) {
    msg!("inside puff out data fields");
    metadata.data.name = puffed_out_string(&metadata.data.name, MAX_NAME_LENGTH);
    msg!("after fist");
    metadata.data.symbol = puffed_out_string(&metadata.data.symbol, MAX_SYMBOL_LENGTH);
    msg!("after second");
    metadata.data.uri = puffed_out_string(&metadata.data.uri, MAX_URI_LENGTH);
    msg!("after thrid");
}

/// Pads the string to the desired size with `0u8`s.
/// NOTE: it is assumed that the string's size is never larger than the given size.
pub fn puffed_out_string(s: &String, size: usize) -> String {
    msg!("input: {:?}",s);
    let mut array_of_zeroes = vec![];
    let puff_amount = size - s.len();
    while array_of_zeroes.len() < puff_amount {
        array_of_zeroes.push(0u8);
    }
    msg!("before unwraping");
    s.clone() + std::str::from_utf8(&array_of_zeroes).unwrap()
}

