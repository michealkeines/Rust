
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    Self::process_with_constraints(program_id, accounts, input, &SWAP_CONSTRAINTS)
}

#[cfg_attr(feature = "fuzz", derive(Arbitrary))]
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Swap {
    /// SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub minimum_amount_out: u64,
}

pub fn process_with_constraints(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
    swap_constraints: &Option<SwapConstraints>,
) -> ProgramResult {
    let instruction = SwapInstruction::unpack(input)?;
    match instruction {
        SwapInstruction::Swap(Swap {
            amount_in,
            minimum_amount_out,
        }) => {
            msg!("Instruction: Swap");
            Self::process_swap(program_id, amount_in, minimum_amount_out, accounts)
        }

    }
}

pub fn process_swap(
    program_id: &Pubkey,
    amount_in: u64,
    minimum_amount_out: u64,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let swap_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let user_transfer_authority_info = next_account_info(account_info_iter)?;
    let source_info = next_account_info(account_info_iter)?;
    let swap_source_info = next_account_info(account_info_iter)?;
    let swap_destination_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;
    let pool_mint_info = next_account_info(account_info_iter)?;
    let pool_fee_account_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
}