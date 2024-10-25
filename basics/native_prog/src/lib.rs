use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    entrypoint::ProgramResult,
     msg
};

use solana_program::entrypoint;


entrypoint!(process_instruction);

pub fn process_instruction(
    _pubkey: &Pubkey,
    _account_infos: &[AccountInfo], 
    _instruction_data: &[u8]
)-> ProgramResult {
    msg!("Helloo world");
    Ok(())
}