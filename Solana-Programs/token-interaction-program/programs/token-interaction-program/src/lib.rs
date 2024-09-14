use anchor_lang::prelude::*;

declare_id!("6BKjcyasHYNtsixnUz1L1B7iM399xzDdSTvueSkE82hx");

#[program]
pub mod token_interaction_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
