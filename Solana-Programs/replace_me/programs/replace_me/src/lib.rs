use anchor_lang::prelude::*;

declare_id!("B2m97h252gtTbcfVCe3NCQzWs2SL8oNechL4GxvHEENx");

#[program]
pub mod replace_me {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
