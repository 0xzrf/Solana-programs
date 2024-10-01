use crate::state::UserData;
use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct CloseAccount<'info>{
    #[account[mut]]
    pub user: Signer<'info>,
    #[account[
        mut,
        seeds = [b"user_data", user.key().as_ref()],
        bump = account.bump,
        close = user // This closes the account and sends the lamports to user
    ]]
    pub account: Account<'info, UserData>,
}


pub fn close_account(_ctx: Context<CloseAccount>)-> Result<()> {
    Ok(())
}