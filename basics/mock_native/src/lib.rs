use solana_program::{
    account_info::{next_account_info, AccountInfo}, declare_id, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey
};
use borsh::{BorshDeserialize, BorshSerialize};
pub mod states; // declares the existance of states in the current folder
use states::*; // imports everything from the states file present in the current directory


// This conditionally compiles the code below after in the compilation process based on whether the condition is true, in this case, if no-entrypoint feature is enabled
// then the code below won't compile, and the not function just inverts the condition
#[cfg(not(feature = "no-entrypoint"))] 
use solana_program::entrypoint;

#[cfg(not(feature = "no-entrypoint"))] 
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey, // program_id of the program
    account_info: &[AccountInfo], // array of accounts in the instruction
    instruction_data: &[u8] 
) -> ProgramResult {
    let (_, instruction_data_inner) = instruction_data.split_at(1);
    match instruction_data_inner[0] {
        0 => {
            process_increment_counter(account_info, instruction_data_inner)?;
        }
        _ => {
            msg!("Error: Unknown error")
        }
    }    
    Ok(())
}

pub fn process_increment_counter(
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();

    let counter_account_data = next_account_info(account_iter)?;

    assert!(
        counter_account_data.is_writable,
        "The account ain't writable man!"
    ); // Making sure that the account is writable

    let mut counter = Counter::try_from_slice(&counter_account_data.try_borrow_mut_data()?)?;
    counter.count += 1;
    
    counter.serialize(&mut *counter_account_data.data.borrow_mut())?;
    Ok(())
}