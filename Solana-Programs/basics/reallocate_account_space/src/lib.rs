use anchor_lang::prelude::*;

declare_id!("4TU1ZZE54nfwb3jP5pZPBgQcQWcshPXU2Cci5PvfDo4Y");

#[program]
pub mod mock {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,input_len: String, user_id: u64) -> Result<()> {
        ctx.accounts.user_account.set_inner(Message{
            user_id,
            user_name: input_len
        });
        Ok(())
    }

    pub fn update(ctx: Context<Update>, input_len: String) -> Result<()> {
        ctx.accounts.user_account.user_name = input_len;
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(input_len:String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        space = Message::input_space(input_len.len()),
        payer = user
    )]
    pub user_account: Account<'info,Message>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(input_len: String)]
pub struct Update<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        realloc = Message::input_space(input_len.len()),
        realloc::payer = user,
        realloc::zero = true
    )]
    pub user_account: Account<'info, Message>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Message{
    pub user_id: u64,
    pub user_name: String
}

impl Message {
    pub fn input_space(input_len: usize) -> usize {
        let return_val = 8 + 8 + input_len;
        return return_val;
    }
}