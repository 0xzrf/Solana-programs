use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey, msg
};

pub mod process_instruction;
pub mod states;
pub mod instructions;

use process_instruction::process_instruction;


entrypoint!(process_instruction);
