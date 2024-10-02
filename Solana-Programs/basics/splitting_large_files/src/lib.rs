use anchor_lang::prelude::*;

pub mod instructions;
pub mod errors;
pub mod state;

use crate::instructions::{};

declare_id!("BPjq2ueaJsrjSse416hh9wx6981B4EAvGHvp5cqxSKyP");

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
