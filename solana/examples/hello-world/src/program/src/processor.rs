use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::instruction::GreetingAccount;

pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    _input: &[u8]
) -> ProgramResult {
    msg!("Hello from processor");
    let accounts_iter = &mut accounts.iter();

    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("program is not correct");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;

    greeting_account.counter += 1;

    greeting_account.serialize(&mut (&mut account.data.borrow_mut()[..]))?;

    msg!("Greeted {} times", greeting_account.counter);
    Ok(())
}