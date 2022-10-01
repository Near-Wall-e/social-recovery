use std::fmt;

use near_sdk::serde::{Deserialize, Serialize};

/// This spec can be treated like a version of the standard.
pub const METADATA_SPEC: &str = "1.0.0";
/// This is the name of the NFT standard we're using
pub const STANDARD_NAME: &str = "alert-contract";

/// Enum that represents the data type of the EventLog.
/// The enum can either be an NftMint or an NftTransfer.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum Notification {
    RecoverAccount(RecoverAccountNotify),
    CancelRecover(CancelRecoverNotify),
}

/// Interface to capture data about an event
///
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RecoverAccountNotify {
    pub account: String,
    pub recoverer: String,
    pub recover_pk: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CancelRecoverNotify {
    pub account: String,
    pub recover_pk: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification() {
        let event = EventLog {
            // Standard name ("nep171").
            standard: STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: METADATA_SPEC.to_string(),
            event: Notification::RecoverAccount(RecoverAccountNotify {
                account: "TestAccount".to_string(),
                recoverer: "Friend".to_string(),
                recover_pk: "deaddeaddead".to_string(),
            })
        };
        println!("{:?}", &event);
        assert_eq!(
            r#"EVENT_JSON:{"standard":"alert-contract","version":"1.0.0","event":"recover_account","data":[{"account":"TestAccount","recoverer":"Friend","recover_pk":"deaddeaddead"}]}"#,
            format!("{}", event),
        );
        let event = EventLog {
            // Standard name ("nep171").
            standard: STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: METADATA_SPEC.to_string(),
            event: Notification::CancelRecover(CancelRecoverNotify {
                account: "TestAccount".to_string(),
                recover_pk: "deaddeaddead".to_string(),
            })
        };
        println!("{:?}", &event);
        assert_eq!(
            r#"EVENT_JSON:{"standard":"alert-contract","version":"1.0.0","event":"cancel_recover","data":[{"account":"TestAccount","recover_pk":"deaddeaddead"}]}"#,
            format!("{}", event),
        );
    }
}
