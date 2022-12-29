use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct Friend {
    pub account_id: AccountId,
    pub name: String,
    pub image: String,
    pub topic: String,
    pub hash: String,
}

impl Friend{
    pub fn new(account_id: AccountId, name: String, image: String, topic: String, hash: String) -> Self{
        Self {
            account_id,
            name,
            image,
            topic,
            hash,
        }
    }
}