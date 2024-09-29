use anchor_lang::prelude::*;

#[account]
pub struct UserOrder {
    pub user: Pubkey,
    pub order_no: u64
}