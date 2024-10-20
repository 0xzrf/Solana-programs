use borsh::{
    BorshDeserialize,
    BorshSerialize
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AddressInfo {
    pub name: String,
    pub user_id: u8
}


impl AddressInfo{
    pub fn new(name: String, user_id: u8)-> AddressInfo {
        return AddressInfo {
            name,
            user_id
        }
    }
}