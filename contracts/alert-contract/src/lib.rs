/*
* Example smart contract written in RUST
*
* Learn more about writing NEAR smart contracts with Rust:
* https://near-docs.io/develop/Contract
*
*/

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};
use shares::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AlertContract {}

impl Default for AlertContract {
    fn default() -> Self {
        Self {}
    }
}

#[near_bindgen]
impl AlertContract {
    pub fn notify(&self, notification: Notification) {
        let predecessor_account_id = env::predecessor_account_id().to_string();
        let notification = match notification {
            Notification::RecoverAccount(mut n) => {
                n.account = predecessor_account_id;
                Notification::RecoverAccount(n)
            },
            Notification::CancelRecover(mut n) => {
                n.account = predecessor_account_id;
                Notification::CancelRecover(n)
            },
            _ => panic!("Wrong notification.")
        };
        let event = EventLog {
            standard: STANDARD_NAME.to_string(),
            version: METADATA_SPEC.to_string(),
            event: notification
        };
        log!("{}", event.to_string())
    }
}

/*
* The rest of this file holds the inline tests for the code above
* Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
*/
#[cfg(test)]
mod tests {
    use super::*;
}
