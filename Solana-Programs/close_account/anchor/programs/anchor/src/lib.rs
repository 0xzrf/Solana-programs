use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("Ggw78LnYS3szTgykUaqtLZ4cP3PPRJsCBFMD6Qthj7Am");

#[program]
pub mod anchor {
    use super::*;

    pub fn create_acc(_ctx: Context<CreateAccount>, user_id: u64, user_email: String, password: String) -> Result<()> {
        create_account(_ctx, user_id, user_email, password)?;        
        Ok(())
    }

    pub fn close_acc(_ctx: Context<CloseAccount>) -> Result<()> {
        close_account(_ctx)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
