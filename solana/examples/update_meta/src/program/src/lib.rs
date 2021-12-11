
pub mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod error;
pub mod utils;
pub mod deprecated_instruction;


// #[cfg(test)]
// mod test {
//     use super::*;
//     use std::mem;
//     use solana_program::{
//         account_info::{next_account_info, AccountInfo},
//         entrypoint::ProgramResult,
//         msg,
//         program_error::ProgramError,
//         pubkey::Pubkey,
//     };

//     #[test]
//     fn test_sanity() {
//         let program_id = Pubkey::default();
//         let key = Pubkey::default();

//     }
// }