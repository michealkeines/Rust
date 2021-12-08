use borsh::{BorshDeserialize, BorshSerialize};
use crate::{
    state::{Creator, Data, EDITION, EDITION_MARKER_BIT_SIZE, PREFIX}
};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct UpdateMetadataAccountArgs {
    pub data: Option<Data>,
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
}


#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum MetadataInstruction {
    UpdateMetadataAccount(UpdateMetadataAccountArgs)
    // UpdateMetadataAccount: UpdateMetadataAccountArgs
}
















// pub fn update_metadata_accounts(
//     program_id: Pubkey,
//     metadata_account: Pubkey,
//     update_authority: Pubkey,
//     new_update_authority: Option<Pubkey>,
//     data: Option<Data>,
//     primary_sale_happened: Option<bool>,
// ) -> Instruction {
//     Instruction {
//         program_id,
//         accounts: vec![
//             AccountMeta::new(metadata_account, false),
//             AccountMeta::new_readonly(update_authority, true),
//         ],
//         data: MetadataInstruction::UpdateMetadataAccount(UpdateMetadataAccountArgs {
//             data,
//             update_authority: new_update_authority,
//             primary_sale_happened,
//         })
//         .try_to_vec()
//         .unwrap(),
//     }
// }