use anchor_lang::prelude::*;

declare_id!("9AwFcxMqCukuKytRkZvodGxcSJZConPf1eFwYiKc6GTv");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world");

        msg!("Your program Id: {}", &id());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
