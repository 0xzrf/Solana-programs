use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::states::User;
use borsh::BorshSerialize;

pub fn create(program_id: &Pubkey, account_infos: &[AccountInfo], data: User) -> ProgramResult {
    let account_iter = &mut account_infos.iter();
    let payer = next_account_info(account_iter)?;
    let target_account = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    let account_span = (data.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    let seeds = &[User::seed_prefix, payer.key.as_ref()];

    let (_, bump) = Pubkey::find_program_address(seeds, program_id);

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            target_account.key,
            lamports_required,
            account_span as u64,
            payer.key,
        ),
        &[
            payer.clone(),
            target_account.clone(),
            system_program.clone(),
        ],
        &[&[User::seed_prefix, payer.key.as_ref(), &[bump]]],
    )?;

    data.serialize(&mut &mut target_account.data.borrow_mut()[..])?;

    Ok(())
}
