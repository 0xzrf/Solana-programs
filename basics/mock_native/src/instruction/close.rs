use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, program::invoke_signed, rent::Rent, system_instruction, sysvar::Sysvar
};



pub fn close(
    account_infos: &[AccountInfo]
) -> ProgramResult {

    let account_iter = &mut account_infos.iter();

    let payer = next_account_info(account_iter)?;
    let target_account = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    let account_span = 0usize; // calculating the actual account size instead of the target account to see what is the rent expempt money we need for this   

    let diff = target_account.lamports();

    **target_account.lamports.borrow_mut() -= diff;
    **payer.lamports.borrow_mut() += diff;

    // Realloc the account to zero usize, meaning just remove the account and it's fund from existance
    target_account.realloc(account_span,true)?;

    target_account.assign(system_program.key);

    Ok(())
}