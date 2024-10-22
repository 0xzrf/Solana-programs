use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey, system_instruction
};

use crate::states::User;

pub fn create(
    program_id: &Pubkey,
    account_infos: &[AccountInfo],
    data: User
) -> ProgramResult {
    Ok(())
}