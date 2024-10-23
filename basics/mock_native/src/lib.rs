use processor::process_instruction;
use solana_program::entrypoint;

pub mod states;
pub mod instruction;
pub mod processor;

entrypoint!(process_instruction);