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



// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(test)]
// mod tests {
//     use near_sdk::test_utils::{accounts, VMContextBuilder};
//     use near_sdk::testing_env;

//     use super::*;

//     // Allows for modifying the environment of the mocked blockchain
//     fn get_context(signer_account_id: AccountId) -> VMContextBuilder {
//         let mut builder = VMContextBuilder::new();
//         builder
//             .current_account_id(accounts(0))
//             .signer_account_id(signer_account_id.clone())
//             .predecessor_account_id(signer_account_id);
//         builder
//     }

//     #[test]
//     fn register_account() {
//         let context = get_context(AccountId::try_from("wx.testnet".to_string()).unwrap());
//         // Initialize the mocked blockchain
//         testing_env!(context.build());

//         let mut account = Account::default();
//         let result = account.register("wangxin".to_string(), "http://aaa.jpg".to_string());
//         assert_eq!(
//             result,
//             true
//         );
//     }

//     #[test]
//     fn test_add_friend() {
//         let mut context = get_context(AccountId::try_from("wx.testnet".to_string()).unwrap());
//         // Initialize the mocked blockchain
//         testing_env!(context.build());
//         let mut account = Account::default();
//         account.register("wangxin".to_string(), "http://wx.jpg".to_string());

//         context.signer_account_id(AccountId::try_from("suzhe.testnet".to_string()).unwrap());
//         testing_env!(context.build());
//         account.register("suzhe".to_string(), "http://suzhe.jpg".to_string());

//         let result = account.add_friend(AccountId::try_from("wx.testnet".to_string()).unwrap());

//         assert_eq!(
//             result,
//             true
//         );
//     }

//     #[test]
//     fn test_get_friends() {
//         let mut context = get_context(AccountId::try_from("wx.testnet".to_string()).unwrap());
//         // Initialize the mocked blockchain
//         testing_env!(context.build());
//         let mut account = Account::default();
//         account.register("wangxin".to_string(), "http://wx.jpg".to_string());

//         context.signer_account_id(AccountId::try_from("suzhe.testnet".to_string()).unwrap());
//         testing_env!(context.build());
//         account.register("suzhe".to_string(), "http://suzhe.jpg".to_string());

//         account.add_friend(AccountId::try_from("wx.testnet".to_string()).unwrap());

//         let binding = account.get_friend_list();
//         let friend_name = binding.get(0).unwrap();

//         assert_eq!(
//             "wangxin",
//             friend_name.name
//         );
//     }

// }