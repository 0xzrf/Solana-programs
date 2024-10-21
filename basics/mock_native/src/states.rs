use borsh::{
    BorshDeserialize,
    BorshSerialize
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Counter {
    pub count: u8
}

