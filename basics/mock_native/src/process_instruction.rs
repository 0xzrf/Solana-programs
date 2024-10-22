use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey
};

use crate::states::User;
use crate::instruction::{
    close::close,
    create::create
};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum CreateOrClose {
    Create(User),
    Close
}

pub fn process_instructions(
    program_id: &Pubkey,
    account_infos: &[AccountInfo],
    instruction_data: &[u8]
)-> ProgramResult {

    let instruction = CreateOrClose::try_from_slice(instruction_data)?;
    match instruction {
        CreateOrClose::Create(data) => create(program_id, account_infos, data)?,
        CreateOrClose::Close => close(account_infos)?
    }

    Ok(())
}