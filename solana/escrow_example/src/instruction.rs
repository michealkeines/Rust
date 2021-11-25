use std::convert::TryFrom;
use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstuction {
    InitEscrow { amount: u64 }
}

impl EscrowInstuction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> { // public API
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        }
        )
    }
    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> { // Private method
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes) // grab first 8 bytes and arrange them in bigindean way
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}