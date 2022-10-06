/*
* Example smart contract written in RUST
*
* Learn more about writing NEAR smart contracts with Rust:
* https://near-docs.io/develop/Contract
*
*/

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, Promise, PublicKey};
use shares::{Notification, RecoverAccountNotify};
use std::collections::{HashMap, HashSet};

const MIN_RECOVERY_TIMEOUT: u64 = 60 * 60 * 24 * 7; // WEEK SHOULD BE A MINIMUM
const MAX_RECOVERY_TIMEOUT: u64 = 60 * 60 * 24 * 365; // YEAR SHOULD BE A MAXIMUM
const ALERT_CONTRACT_ACCOUNT_ID: &str = "alert.nearuaguild.testnet"; // NOTIFICATION CONTRACT
const GAS_FOR_NOTIFICATION: Gas = Gas(150_000_000_000_000);

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub enum State {
    NORECOVER,     // no recover
    APPROVING,     // collecting recover approves from trusted parties
    TIMELOCK(u64), // recover approved, but timelock required to wait for possible recover cancel.
}

/// The struct is used only for return data.
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct RecoverStatus {
    state: State,
    approved_accounts: HashMap<AccountId, PublicKey>,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct RecoverConfig {
    threshold: usize,
    timeout: u64,
    accounts: HashSet<AccountId>,
}

#[ext_contract(ext_linkdrop)]
pub trait ExtLinkDropAlarmContract {
    fn notify(&mut self, notification: Notification) -> Promise;
}

#[near_bindgen]
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct RecoveryContract {
    // Used only for dev, for real deploy loggin will be inside the same contract.
    alarm_contract: AccountId,
    config: RecoverConfig,
    status: RecoverStatus,
    approved_accounts: HashMap<AccountId, PublicKey>,
}

impl Default for RecoveryContract {
    fn default() -> Self {
        Self {
            alarm_contract: AccountId::new_unchecked(ALERT_CONTRACT_ACCOUNT_ID.to_string()),
            config: RecoverConfig {
                threshold: 0,
                timeout: 0,
                accounts: HashSet::new(),
            },
            status: RecoverStatus {
                state: State::NORECOVER,
                approved_accounts: HashMap::new(),
            },
            approved_accounts: HashMap::new(),
        }
    }
}

#[near_bindgen]
impl RecoveryContract {
    #[init]
    #[private]
    pub fn init(alarm_contract: AccountId, config: RecoverConfig) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            alarm_contract,
            config,
            status: RecoverStatus {
                state: State::NORECOVER,
                approved_accounts: HashMap::new(),
            },
            approved_accounts: HashMap::new(),
        }
    }

    pub fn get_self(&mut self) -> RecoveryContract {
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

    pub fn approve_public_key(&mut self, pk: PublicKey) {
        let approver = env::predecessor_account_id();
        if !self.config.accounts.contains(&approver) {
            panic!("Only trusted approvers can participate.");
        }
        if let State::TIMELOCK(_) = self.status.state {
            panic!("Recovery passed approve state.")
        }
        self.approved_accounts.insert(approver.clone(), pk.clone());
        self.status.state = State::APPROVING;
        ext_linkdrop::ext(self.alarm_contract.clone())
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
            self.status.state = State::TIMELOCK(env::block_timestamp() + self.config.timeout);
            // TODO notify alert contract
        }
    }

    pub fn cancel_recovery(&mut self) {
        if env::predecessor_account_id() != env::current_account_id() {
            panic!("Only account owner may cancel.");
        }
        // clean recover
        self.status.approved_accounts = HashMap::new();
        self.status.state = State::NORECOVER;
    }

    pub fn commit_recover(&mut self) {
        let timeout = match self.status.state {
            State::TIMELOCK(timeout) => timeout,
            _ => panic!("Wrong state to commit."),
        };
        if timeout > env::block_timestamp() {
            panic!("You should wait for timeout: {}", timeout);
        }
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
