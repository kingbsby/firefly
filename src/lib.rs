mod friend;
mod account_info;
mod room;
pub use crate::friend::*;
pub use crate::account_info::*;
pub use crate::room::*;

use account_info::room::Room;
use near_sdk::BorshStorageKey;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{near_bindgen, AccountId};

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKeys {
    Accounts,
    AccountFriends { account_id: AccountId },
    Rooms,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    accounts: LookupMap<AccountId, AccountInfo>,
    rooms: UnorderedMap<u32, Room>,
    room_num: u32,
}

impl Default for Contract{
    fn default() -> Self {
        Self {
            accounts: LookupMap::new(StorageKeys::Accounts),
            rooms: UnorderedMap::new(StorageKeys::Rooms),
            room_num: 0,
        }
    }
}

impl Contract{
    fn accounts(&self) -> &LookupMap<AccountId, AccountInfo> {
        &self.accounts
    }

    fn accounts_mut(&mut self) -> &mut LookupMap<AccountId, AccountInfo> {
        &mut self.accounts
    }

    fn rooms(&self) -> &UnorderedMap<u32, Room> {
        &self.rooms
    }

    fn rooms_mut(&mut self) -> &mut UnorderedMap<u32, Room> {
        &mut self.rooms
    }

    fn get_next_room_id(&mut self) -> u32 {
        self.room_num = self.room_num + 1;
        self.room_num
    }
}