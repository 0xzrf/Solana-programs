use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    entrypoint::ProgramResult,
    program_error::ProgramError
};

use crate::states::AddressInfo;


pub fn process_instruction(
    program_id: &Pubkey,
    account_info: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    if let Ok(address_info) = AddressInfo::try_from_slice(instruction_data) {
        
    };

    Err(ProgramError::InvalidInstructionData)
}