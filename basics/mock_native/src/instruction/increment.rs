use crate::states::PageVisits;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult};


pub fn increment (
    accounts: &[AccountInfo]
) -> ProgramResult {

    // Define the account from the instruction
    let accounts_iter = &mut accounts.iter(); // create the iter
    let page_visits = next_account_info(accounts_iter)?;;

    let page_visit = &mut PageVisits::try_from_slice(&page_visits.data.borrow())?;// Deserializing the page_visit pda from the instrucition

    page_visit.increment();
    page_visit.serialize(&mut &mut page_visits.data.borrow_mut()[..])?;

    Ok(())
}