use anchor_lang::prelude::*;

declare_id!("3AYVYRrRcz7uHr6bg1FcKRHAdHH5vai8CGEjJkWKRUo8");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, user_id: u64, order_number: u64) -> Result<()> {
        *ctx.accounts.context = Order{
            user_id: user_id,
            order_number: order_number
        };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account[mut]]
    pub signer: Signer<'info>,
    #[account[init, payer = signer, space = 8 + 8 + 8]]
    pub context: Account<'info, Order>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Order {
    pub user_id: u64,
    pub order_number: u64
}
