use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    entrypoint::ProgramResult
};

use crate::states::*;
use crate::instruction::{create::create_pda, increment::increment};

pub fn process_instruction(
    program_id: &Pubkey,
    account_infos: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {

    if let Ok(page_visits) = PageVisits::try_from_slice(instruction_data) {
        create_pda(program_id, account_infos, page_visits)?;
    } 

    if IncrementPageVists::try_from_slice(instruction_data).is_ok() {
        increment(account_infos)?;
    }

    Ok(())
}