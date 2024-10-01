use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserData{
    pub user_id: u64,
    #[max_len(40)]
    pub user_email: String,
    #[max_len(20)]
    pub password: String
}