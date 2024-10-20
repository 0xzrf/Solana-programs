use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    entrypoint::ProgramResult,
    program_error::ProgramError
};

use crate::states::AddressInfo;
use crate::instructions::create_account::create_account;

pub fn process_instruction(
    program_id: &Pubkey,
    account_info: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    if let Ok(address_info) = AddressInfo::try_from_slice(instruction_data) {
        return create_account(program_id, account_info, address_info)?;
    };

    Err(ProgramError::InvalidInstructionData)
}