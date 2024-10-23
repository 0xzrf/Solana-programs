use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar
};
use borsh::BorshSerialize;


use crate::states::PageVisits;


pub fn create_pda(
    program_id: &Pubkey,
    account_infos: &[AccountInfo],
    data: PageVisits
)-> ProgramResult {

    let account_iter =  &mut account_infos.iter();
    
    let payer = next_account_info(account_iter)?;
    let page_visits_account = next_account_info(account_iter)?;
    // let user = next_account_info(account_iter)?; // optional if you want
    let payer = next_account_info(account_iter)?;
    let system_account =  next_account_info(account_iter)?;

    let acocunt_span = (data.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(acocunt_span);

    // Create the pda with signer privlages, and provide the seeds you'd like the pda to have
    invoke_signed(&system_instruction::create_account(payer.key, 
        page_visits_account.key,
        lamports_required, 
        acocunt_span as u64,
        payer.key), &[
            page_visits_account.clone(),
            payer.clone(),
            system_account.clone()
        ], &[&[
            PageVisits::SEED_PREFIX.as_bytes(),
            payer.key.as_ref(),
            &[data.bump]
        ]])?;

    Ok(())
}