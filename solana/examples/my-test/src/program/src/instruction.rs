
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};
use crate::{
    state::{ Creator, Data }
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Test1Args {
    pub data: Data,
    pub is_mutable: bool
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Test2Args {
    pub data: Option<Data>,
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>
}


#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum MetadataInstruction {
    Test1(Test1Args),
    Test2(Test2Args)
}