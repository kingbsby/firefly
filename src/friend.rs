use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct Friend {
    pub name: String,
    pub image: String,
    pub topic: String,
    pub hash: String,
}

impl Friend{
    pub fn new(name: String, image: String, topic: String, hash: String) -> Self{
        Self {
            name,
            image,
            topic,
            hash,
        }
    }
}