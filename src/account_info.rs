pub use crate::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{env, AccountId};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AccountInfo {
    pub name: String,
    pub image: String,
    pub friends: UnorderedMap<AccountId, String>,
    pub hash: String,
}

impl AccountInfo{
    pub fn new(account_id: &AccountId, name: String, image: String) -> Self{
        Self {
            name,
            image,
            friends: UnorderedMap::new(StorageKeys::AccountFriends { account_id: account_id.clone() }),
            hash: "".to_string(),
        }
    }

    pub fn update_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn add_friend(&mut self, friend_account_id: &AccountId, topic:&String) {
        self.friends.insert(friend_account_id, topic);
    }

    pub fn remove_friend(&mut self, friend_account_id: &AccountId) {
        self.friends.remove(friend_account_id);
    }
}

#[near_bindgen]
impl Contract{
    pub fn register(&mut self, name: String, image: String) -> bool {
        let account_id = env::signer_account_id();
        assert!(!self.accounts().contains_key(&account_id), "Account already exists");
        let account_info = AccountInfo::new(&account_id, name, image);
        self.accounts_mut().insert(&account_id, &account_info);
        true
    }

    pub fn login(&mut self, hash: String) -> Vec<String> {
        let account_id = env::signer_account_id();
        assert!(self.accounts().contains_key(&account_id), "Account {0} not registered", &account_id);
        let mut account_info = self.accounts.get(&account_id).unwrap();
        account_info.update_hash(hash);
        self.accounts_mut().insert(&account_id, &account_info);
        
        vec![account_info.name, account_info.image]
    }

    pub fn get_friend_list(&self) ->  Vec<Friend>{
        let account_id = env::signer_account_id();
        println!("get_friend_list account id {}", account_id);
        assert!(self.accounts().contains_key(&account_id), "Account {account_id} not registered");

        // get friends and topic of current account 
        let friend_list: UnorderedMap<AccountId, String> = self.accounts().get(&account_id).expect("can not find the account").friends;

        let mut vec_friend: Vec<Friend> = Vec::new();
        // get account info for friend_list, generate Friend object
        for friend_account_id in friend_list.keys() {
            let friend_info = self.accounts().get(&friend_account_id).expect("can not find the friend account '{friend_account_id}'");
            vec_friend.push(
                Friend{
                    name: friend_info.name,
                    image: friend_info.image,
                    topic: friend_list.get(&friend_account_id).expect("friend list not contained the accountId"),
                    hash: friend_info.hash,
                }
            )
        };

        vec_friend
    }

    pub fn add_friend(&mut self, friend_account_id: AccountId) -> bool{
        let account_id = env::signer_account_id();
        // Calculate the value of topic (hash of account_id and friend_account_id)
        let mut concat_str = account_id.to_string();
        concat_str.push_str(friend_account_id.as_str());
        let mut hasher = DefaultHasher::new();
        concat_str.hash(&mut hasher);
        let hash_str = hasher.finish().to_string();

        self.add_friend_to_account(&account_id, &friend_account_id, &hash_str);
        self.add_friend_to_account(&friend_account_id, &account_id, &hash_str);
        true
    }

    fn add_friend_to_account(&mut self, account_id: &AccountId, friend_account_id: &AccountId, topic: &String) {
        println!("account id:{}, friend_account_id:{}", &account_id, &friend_account_id);
        assert!(self.accounts().contains_key(&account_id), "Account {account_id} not registered");
        assert!(self.accounts().contains_key(&friend_account_id), "Account {friend_account_id} not registered");
        // get friends object
        let mut account_info = self.accounts().get(&account_id).expect("can not find the account");
        account_info.add_friend(friend_account_id, topic);

        self.accounts_mut().insert(&account_id, &account_info);
    }

    pub fn remove_friend(&mut self, friend_account_id: AccountId) -> bool{
        let account_id = env::signer_account_id();
        self.remove_friend_from_account(&account_id, &friend_account_id);
        self.remove_friend_from_account(&friend_account_id, &account_id);
        true
    }

    fn remove_friend_from_account(&mut self, account_id: &AccountId, friend_account_id: &AccountId){
        // get friends object
        let mut account_info = self.accounts().get(&account_id).expect("can not find the account");

        // modify friends to add new friend
        account_info.remove_friend(&friend_account_id);
        self.accounts_mut().insert(&account_id, &account_info);
    }
}