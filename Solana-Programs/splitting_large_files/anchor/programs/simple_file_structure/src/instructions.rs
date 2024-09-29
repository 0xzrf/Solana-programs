use anchor_lang::prelude::*;
use crate::errors::ErrorCode;//This imports ErrorCode from the errors.rs file, which is present in the root director

use crate::state::UserOrder; 

pub mod simple_file_struct{
    use super::*;
    
    pub fn initialize_an_order(ctx: Context<InitializeOrder>, order: String) -> Result<()> {
        let new_order = &mut ctx.accounts.order;
        let payer = &mut ctx.accounts.payer;
        if order.chars().count() > 10 {
            return Err(ErrorCode::InvalidOrder.into());
        }

        new_order.order = order; // ik pretty weird
        new_order.user = *payer.key;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeOrder<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, seeds=[b"order",payer.key().as_ref()], bump, space = UserOrder::LEN)]
    pub order: Account<'info, UserOrder>,
    pub system_program: Program<'info, System>
}
