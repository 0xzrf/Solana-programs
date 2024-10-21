use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, pubkey::Pubkey, rent::Rent, program::invoke, system_instruction, sysvar::Sysvar, msg
};
use borsh::{BorshDeserialize, BorshSerialize};

pub mod states;
use states::UserData;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    // Define your seeds
    let seeds: &[&[u8]] = &[b"my_seed", b"another_seed"];

    // Derive the PDA
    let derived_pda = derive_pda(seeds, program_id);

    // Log the derived PDA (for debugging purposes)
    msg!("Derived PDA: {:?}", derived_pda);

    let mut account_info_iter = accounts.iter();

    let payer_account = next_account_info(&mut account_info_iter)?;

    let pda_account_info = next_account_info(&mut account_info_iter)?;
    let rent = &Rent::get()?;

    if pda_account_info.data_is_empty() {
        let space_required = std::mem::size_of::<UserData>();

        let lamports_required = rent.minimum_balance(space_required);

        invoke(
            &system_instruction::create_account(
                payer_account.key,
                pda_account_info.key,
                lamports_required,
                space_required as u64,
                program_id,
            ),
            &[payer_account.clone(), pda_account_info.clone()],
        )?;

        msg!("PDA account created.");

        // Initialize UserData
        let user_data = UserData { counter: 0 };
        user_data.serialize(&mut *pda_account_info.try_borrow_mut_data()?)?;
    } else {
        msg!("PDA account already exists.");
    }

    let mut pda_data = pda_account_info.try_borrow_mut_data()?;
    
    let mut user_data = UserData::try_from_slice(&pda_data)?;
    user_data.counter = user_data.counter.checked_add(1).unwrap_or(u8::MAX);

    // Serialize the modified data back into the PDA account
    user_data.serialize(&mut *pda_data)?;

    msg!("Counter state incremented to {:?}", user_data.counter);

    Ok(())
}

pub fn derive_pda(seeds: &[&[u8]], program_id: &Pubkey) -> Pubkey {
    let (pda, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    pda
}
