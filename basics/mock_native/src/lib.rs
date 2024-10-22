use process_instruction::process_instructions;
use solana_program::{
    entrypoint
};

pub mod process_instruction;
pub mod states;
pub mod instruction;

entrypoint!(process_instructions);