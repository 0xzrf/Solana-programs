use solana_program::entrypoint;

pub mod process_instruction;
pub mod states;
pub mod instructions;

use process_instruction::process_instruction;


entrypoint!(process_instruction);
