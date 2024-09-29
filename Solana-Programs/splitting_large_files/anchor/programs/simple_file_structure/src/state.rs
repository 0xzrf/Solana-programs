use anchor_lang::prelude::*;

#[account]
pub struct UserOrder {
    pub user: Pubkey,
    pub order: String
}

impl UserOrder {
    pub const LEN: usize = 32 + 8 + 26;
}