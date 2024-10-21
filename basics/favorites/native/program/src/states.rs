use borsh::{
    BorshDeserialize,
    BorshSerialize
};


#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct UserData {
    pub counter: u8
}