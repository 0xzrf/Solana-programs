use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar   
};
use crate::states::AddressInfo;

pub fn create_account(
    program_id: &Pubkey,
    account_info: &[AccountInfo],
    address_info: AddressInfo
) -> ProgramResult {
    let account_iter = &mut account_info.iter();
    let address_info_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    let account_span = (address_info.try_to_vec()?).len();
    
    Ok(())
}