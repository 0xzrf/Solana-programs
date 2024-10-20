use solana_program::{
    account_info::AccountInfo, 
    pubkey::Pubkey,
    msg,
    entrypoint,
    entrypoint::ProgramResult
};

// Tells the solana program that this is the entrypoint, or the function to run when the instruction is called
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,// Reference to the public Key of the program
    _address: &[AccountInfo],//
    _instruction_data: &[u8]
) -> ProgramResult {

    msg!("Hello world");

    Ok(())
}
