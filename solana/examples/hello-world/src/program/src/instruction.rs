

use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct GreetingAccount {
    pub counter: u32
}