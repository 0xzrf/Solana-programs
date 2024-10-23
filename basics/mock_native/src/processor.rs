use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    entrypoint::ProgramResult
};

use crate::states::*;

pub fn process_instruction(
    program_id: &Pubkey,
    account_infos: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {

    if let Ok(page_visits) = IncrementPageVists::try_from_slice(instruction_data) {

    } 

    if IncrementPageVists::try_from_slice(instruction_data).is_ok() {

    }

    Ok(())
}