pub use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use near_sdk::{env, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct Room {
    pub id: u32,
    pub name: String,
    pub topic: String,
    pub nft: Vec<String>,
    pub owner: AccountId,
}

impl Room{
    pub fn new(contract: &mut Contract, name: String, nft: Vec<String>) -> Self{
        let account_id = env::signer_account_id();
        let room_id = contract.get_next_room_id();
        // 生成topic，为“room”+id的hash
        let mut topic_str: String = "room".to_string();
        topic_str.push_str(room_id.to_string().as_str());
        let mut hasher = DefaultHasher::new();
        topic_str.hash(&mut hasher);
        let topic = hasher.finish().to_string();
        Self {
            id: room_id,
            name,
            topic,
            nft,
            owner: account_id,
        }
    }
}

#[near_bindgen]
impl Contract{
    pub fn create_room(&mut self, name: String, nft: Vec<String>) -> u32{
        let room = Room::new(self, name, nft);
        let rooms = self.rooms_mut();
        UnorderedMap::insert(rooms, &room.id, &room);
        room.id
    }

    pub fn get_room_list(&self) -> Vec<(u32, Room)>{
        self.rooms().to_vec()
    }

    pub fn destory_room(&mut self, room_id: u32) -> u32{
        let account_id = env::signer_account_id();
        let room = self.rooms().get(&room_id).expect("room id does not exist");
        assert_eq!(account_id, room.owner, "You do not have permission to delete this room");
        self.rooms_mut().remove(&room_id);
        room_id
    }
}