use std::fmt;

use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{PublicKey, AccountId};

pub const METADATA_SPEC: &str = "1.0.0";
pub const STANDARD_NAME: &str = "alert-contract";

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum Notification {
    RecoverAccount(RecoverAccountNotify),
    CancelRecover(CancelRecoverNotify),
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    #[serde(flatten)]
    pub event: Notification,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RecoverAccountNotify {
    pub account: AccountId,
    pub recoverer: AccountId,
    pub recover_pk: PublicKey,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CancelRecoverNotify {
    pub account: AccountId,
    pub recover_pk: PublicKey,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification() {
        let event = EventLog {
            standard: STANDARD_NAME.to_string(),
            version: METADATA_SPEC.to_string(),
            event: Notification::RecoverAccount(RecoverAccountNotify {
                account: AccountId::new_unchecked("test.testnet".to_string()),
                recoverer: AccountId::new_unchecked("friend.testnet".to_string()),
                recover_pk: "ed25519:6E8sCci9badyRkXb3JoRpBj5p8C6Tw41ELDZoiihKEtp".parse().unwrap(),
            })
        };
        println!("{:?}", &event);
        assert_eq!(
            r#"EVENT_JSON:{"standard":"alert-contract","version":"1.0.0","event":"recover_account","data":{"account":"test.testnet","recoverer":"friend.testnet","recover_pk":"ed25519:6E8sCci9badyRkXb3JoRpBj5p8C6Tw41ELDZoiihKEtp"}}"#,
            format!("{}", event),
        );
        let event = EventLog {
            standard: STANDARD_NAME.to_string(),
            version: METADATA_SPEC.to_string(),
            event: Notification::CancelRecover(CancelRecoverNotify {
                account: AccountId::new_unchecked("test.testnet".to_string()),
                recover_pk: "ed25519:6E8sCci9badyRkXb3JoRpBj5p8C6Tw41ELDZoiihKEtp".parse().unwrap(),
            })
        };
        println!("{:?}", &event);
        assert_eq!(
            r#"EVENT_JSON:{"standard":"alert-contract","version":"1.0.0","event":"cancel_recover","data":{"account":"test.testnet","recover_pk":"ed25519:6E8sCci9badyRkXb3JoRpBj5p8C6Tw41ELDZoiihKEtp"}}"#,
            format!("{}", event),
        );
    }
}
