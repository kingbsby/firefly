mod friend;
mod account_info;
pub use crate::friend::*;
pub use crate::account_info::*;

use near_sdk::BorshStorageKey;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{near_bindgen, AccountId};

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKeys {
    Accounts,
    AccountFriends { account_id: AccountId },
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct Contract {
    accounts: LookupMap<AccountId, AccountInfo>,
}

impl Default for Contract{
    fn default() -> Self {
        Self {
            accounts: LookupMap::new(StorageKeys::Accounts),
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
}