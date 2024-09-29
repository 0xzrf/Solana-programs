use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod errors;

use crate::instructions::*;

declare_id!("5GpzEvVJhjy16i1rr6EotK3gMy86zPw1xvtWfyxQuv7g");

#[program]
pub mod simple_file_structure {
    use super::*;

    pub fn initialize_order(ctx: Context<InitializeOrder>, order: String) -> Result<()> {
        simple_file_struct::initialize_an_order(ctx, order)?;
        Ok(())
    }
}
