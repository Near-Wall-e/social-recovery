/*
* Example smart contract written in RUST
*
* Learn more about writing NEAR smart contracts with Rust:
* https://near-docs.io/develop/Contract
*
*/

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{log, env, ext_contract, near_bindgen, AccountId, Gas, Promise, PublicKey};
use shares::{Notification, RecoverAccountNotify, SubscriptionNotify, EventLog, STANDARD_NAME, METADATA_SPEC};
use std::collections::{HashMap, HashSet};

const ALERT_CONTRACT_ACCOUNT_ID: &str = "alert.nearuaguild.testnet"; // NOTIFICATION CONTRACT
const SUBSCRIPTION_RECEIVER_ACCOUNT_ID: &str = "subscription.nearuaguild.testnet";
const MIN_RECOVERY_TIMEOUT: u64 = 60 * 60 * 24 * 7; // WEEK SHOULD BE A MINIMUM
const MAX_RECOVERY_TIMEOUT: u64 = 60 * 60 * 24 * 365; // YEAR SHOULD BE A MAXIMUM
const GAS_FOR_NOTIFICATION: Gas = Gas(150_000_000_000_000);

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub enum RecoveryState {
    NORECOVER,     // no recover
    APPROVING,     // collecting recover approves from trusted parties
    TIMELOCK((u64, PublicKey)), // recover approved, but timelock required to wait for possible recover cancel.
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub enum SubscriptionState {
    NOSUBSCRIPTION,
    ACTIVE,
}

/// The struct is used only for return data.
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct RecoverStatus {
    state: RecoveryState,
    approved_accounts: HashMap<AccountId, PublicKey>,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct RecoverConfig {
    threshold: usize,
    timeout: u64,
    accounts: HashSet<AccountId>,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct SubscriptionStatus {
    id: usize,
    state: SubscriptionState,
    payment_amount: u128,
    next_payment: u64,
    delay: u64,
}

#[ext_contract(ext_linkdrop)]
pub trait ExtLinkDropAlarmContract {
    fn notify(&mut self, notification: Notification) -> Promise;
}

#[near_bindgen]
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct RecoveryContract {
    config: RecoverConfig,
    status: RecoverStatus,
    subscription: SubscriptionStatus,
}

impl Default for RecoveryContract {
    fn default() -> Self {
        Self {
            config: RecoverConfig {
                threshold: 0,
                timeout: 0,
                accounts: HashSet::new(),
            },
            status: RecoverStatus {
                state: RecoveryState::NORECOVER,
                approved_accounts: HashMap::new(),
            },
            subscription: SubscriptionStatus {
                id: 0,
                delay: 0,
                state: SubscriptionState::NOSUBSCRIPTION,
                payment_amount: 0,
                next_payment: 0,
            }
        }
    }
}

#[near_bindgen]
impl RecoveryContract {
    #[init]
    #[private]
    pub fn init(config: RecoverConfig, subscription: SubscriptionStatus) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            config,
            status: RecoverStatus {
                state: RecoveryState::NORECOVER,
                approved_accounts: HashMap::new(),
            },
            subscription,
        }
    }

    pub fn get_self(&self) -> RecoveryContract {
        return self.clone()
    }

    pub fn update_config(&mut self, config: RecoverConfig) {
        if env::predecessor_account_id() != env::current_account_id() {
            panic!("Only account owner can change the config.");
        }
        if config.timeout > MAX_RECOVERY_TIMEOUT || config.timeout < MIN_RECOVERY_TIMEOUT {
            panic!("Wrong timeout range.");
        }
        self.config = config;
    }

    pub fn update_subscription(&mut self, subscription: SubscriptionStatus) {
        if env::predecessor_account_id() != env::current_account_id() {
            panic!("Only account owner can change the subscription.");
        }
        self.subscription = subscription;
    }

    pub fn pay_subscription(&mut self) -> Promise{
        if self.subscription.next_payment > env::block_timestamp() {
            panic!("Can't take subscription. TIMEOUT.");
        }
        // TODO make event
        self.notify(Notification::SubscriptionPayed(SubscriptionNotify{
            account: env::current_account_id(),
            amount:self.subscription.payment_amount,
            sub_id:self.subscription.id,
        }));
        Promise::new(AccountId::new_unchecked(SUBSCRIPTION_RECEIVER_ACCOUNT_ID.to_string())).transfer(self.subscription.payment_amount)
        
    }

    pub fn approve_public_key(&mut self, pk: PublicKey) {
        let approver = env::predecessor_account_id();
        if !self.config.accounts.contains(&approver) {
            panic!("Only trusted approvers can participate.");
        }
        if let RecoveryState::TIMELOCK(_) = self.status.state {
            panic!("Recovery passed approve state.")
        }
        self.status.approved_accounts.insert(approver.clone(), pk.clone());
        self.status.state = RecoveryState::APPROVING;
        ext_linkdrop::ext(AccountId::new_unchecked(ALERT_CONTRACT_ACCOUNT_ID.to_string()))
            .with_static_gas(GAS_FOR_NOTIFICATION) // This amount of gas will be split
            .notify(Notification::RecoverAccount(RecoverAccountNotify {
                account: AccountId::new_unchecked("".to_string()),
                recoverer: approver,
                recover_pk: pk.clone(),
        }));
        let mut approves_count = 0;
        self.status
            .approved_accounts
            .values()
            .for_each(|approved_pk| {
                if approved_pk == &pk {
                    approves_count += 1
                }
            });
        if approves_count >= self.config.threshold {
            self.status.state = RecoveryState::TIMELOCK((env::block_timestamp() + self.config.timeout, pk));
        }
    }

    pub fn cancel_recovery(&mut self) {
        if env::predecessor_account_id() != env::current_account_id() {
            panic!("Only account owner may cancel.");
        }
        // clean recover
        self.status.approved_accounts = HashMap::new();
        self.status.state = RecoveryState::NORECOVER;
    }

    pub fn commit_recover(&mut self) -> Promise {
        let (timeout, pk) = match self.status.state {
            RecoveryState::TIMELOCK((timeout, ref pk)) => (timeout, pk.clone()),
            _ => panic!("Wrong state to commit."),
        };
        if timeout > env::block_timestamp() {
            panic!("You should wait for timeout: {}", timeout);
        }
        Promise::new(env::current_account_id()).add_full_access_key(pk)
    }

    fn notify(&self, notification: Notification) {
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
