use crate::{ processor};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
pubkey::Pubkey,
};

entrypoint!(process_instruction); // Any program or user calls this entrypoint
fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &'a [u8],
) -> ProgramResult {
    // we send this to process_instuction function to find which instruction to process
    if let Err(error) = processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        return Err(error);
    }
    Ok(())
}
