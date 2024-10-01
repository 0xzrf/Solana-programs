use crate::state::UserData;
use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct CreateAccount<'info>{
    #[account[mut]]
    pub user: Signer<'info>,
    #[account[
        init, 
        space = UserData::INIT_SPACE,
        seeds = [b"user_data", user.key().as_ref()],
        bump, 
        payer = user
    ]]
    pub account: Account<'info, UserData>,
    pub system_program: Program<'info, System>
}


pub fn create_account(ctx: Context<CreateAccount>, user_id: u64, user_email: String, password: String)-> Result<()> {
    ctx.accounts.account.set_inner(UserData {
        user_id, 
        user_email,
        password,
        bump: ctx.bumps.account
    });
    Ok(())
}