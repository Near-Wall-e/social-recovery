/*
* Example smart contract written in RUST
*
* Learn more about writing NEAR smart contracts with Rust:
* https://near-docs.io/develop/Contract
*
*/

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct RecoverAccountNotify {
    account: String,
    recoverer: String,
    recover_pk: String,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct CancelRecoverNotify {
    account: String,
    recover_pk: String,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub enum Notification {
    RecoverAccount(RecoverAccountNotify),
    CancelRecover(CancelRecoverNotify),
}

impl ToString for Notification {
    fn to_string(&self) -> String {
        match self {
            Notification::RecoverAccount(recover_n) => {
                format!(
                    r#"{{"account": "{}", "recoverer": "{}", "recover_pk": "{}"}}"#,
                    recover_n.account, recover_n.recoverer, recover_n.recover_pk,
                )
            }
            Notification::CancelRecover(cancel_n) => {
                format!(
                    r#"{{"account": "{}", "recover_pk": "{}"}}"#,
                    cancel_n.account, cancel_n.recover_pk,
                )
            }
        }
    }
}

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
            }
        };
        log!("{}", notification.to_string())
    }
}

/*
* The rest of this file holds the inline tests for the code above
* Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification() {
        let notification = Notification::RecoverAccount(RecoverAccountNotify {
            account: "TestAccount".to_string(),
            recoverer: "Friend".to_string(),
            recover_pk: "deaddeaddead".to_string(),
        });
        println!("{:?}", &notification);
        assert_eq!(
            r#"{"account": "TestAccount", "recoverer": "Friend", "recover_pk": "deaddeaddead"}"#,
            notification.to_string(),
        );

        let notification = Notification::CancelRecover(CancelRecoverNotify {
            account: "TestAccount".to_string(),
            recover_pk: "deaddeaddead".to_string(),
        });
        println!("{:?}", &notification);
        assert_eq!(
            r#"{"account": "TestAccount", "recover_pk": "deaddeaddead"}"#,
            notification.to_string(),
        );
    }
}
