use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed, system_instruction
};



pub fn close(
    account_infos: &[AccountInfo]
) -> ProgramResult {
    Ok(())
}