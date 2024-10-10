use anchor_lang::prelude::*;

declare_id!("9XGrGqgjy9a6q6BWXJ5Y9BN9unMCXyhhnGeCHzV8N9Zd");

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
