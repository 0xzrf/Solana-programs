use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

declare_id!("Ggw78LnYS3szTgykUaqtLZ4cP3PPRJsCBFMD6Qthj7Am");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
