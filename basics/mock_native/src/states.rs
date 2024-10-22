use borsh::{
    BorshDeserialize,
    BorshSerialize
};


#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    pub name: String
}

impl User {
    pub const seed_prefix: &[u8] = b"User";
}