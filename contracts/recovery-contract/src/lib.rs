/*
* Example smart contract written in RUST
*
* Learn more about writing NEAR smart contracts with Rust:
* https://near-docs.io/develop/Contract
*
*/

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, Set, AccountId};
use shares::Notification;
use std::Vec;

const MIN_RECOVERY_TIMEOUT: u64 = 60 * 60 * 24 * 7; // WEEK SHOULD BE A MINIMUM
const MAX_RECOVERY_TIMEOUT: u64 = 60 * 60 * 24 * 365; // YEAR SHOULD BE A MAXIMUM
const ALERT_CONTRACT_ACCOUNT_ID: &str = "alert.nearuaguild.testnet"; // NOTIFICATION CONTRACT

enum State {
    NO_RECOVER, // no recover
    APPROVING, // collecting recover approves from trusted parties
    TIMELOCK(u64), // recover approved, but timelock required to wait for possible recover cancel.
}

/// The struct is used only for return data.
struct RecoverStatus {
    state: State,
    approved_accounts: Vec<AccountId>,
}

struct RecoverConfig {
    threshold: usize,
    timeout: u64,
    accounts: Set<AccountId>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RecoveryContract {
    alarm_contract: AccountId,
    config: RecoverConfig,
    state: RecoverStatus,
    approved_accounts: HashMap<AccountId, PublicKey>,
}

#[ext_contract(ext_linkdrop)]
pub trait ExtLinkDropAlarmContract {
    fn notify(&mut self, notification: Notification) -> Promise;
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
            state: State::NO_RECOVER
            approved_accounts: HashMap::new(),
        }
    }

    pub fn update_config(&mut self, config: RecoverConfig) {
        if env::predecessor_account_id() != env::current_account_id() {
            panic!("Only account owner can change the config.");
        }
        self.config = config;
    }

    pub fn approve_public_key(&mut self, pk: PublicKey) {
        let approver = env::predecessor_account_id();
        if !self.config.accounts.contains(&approver) {
            panic!("Only trusted approvers can participate.");
        }
        if let State::TIMELOCK(_) = self.state {
            panic!("Recovery passed approve state.")
        }
        self.approved_accounts.insert(approver, pk);
        self.state = State::APPROVING;
        ext_linkdrop::ext(self.alarm_contract.clone())
            .with_attached_deposit(reward_amount)
            .with_static_gas(GAS_FOR_ACCOUNT_CREATION) // This amount of gas will be split
            .create_account(new_acc_id.parse().unwrap(), new_pk)
        let mut approves_count = 0;
        self.approved_accounts.values().map(|approved_pk| if approved_pk == pk {approves_count += 1} );
        if approves_count >= self.config.threshold {
            self.state = State::TIMELOCK(env::block_timestamp() + self.config.timeout);
            // TODO notify alert contract
        }
    }

    pub fn cancel_recovery(&mut self) {
        if env::predecessor_account_id() != env::current_account_id() {
            panic!("Only account owner may cancel.");
        }
        // clean recover
        self.approved_accounts = HashMap::new();
        self.state = State::NO_RECOVER;
    }

    pub fn commit_recover(&mut self) {
        let timeout = match self.state {
            State::TIMELOCK(timeout) => timeout,
            _ => panic!("Wrong state to commit.")
        }
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
