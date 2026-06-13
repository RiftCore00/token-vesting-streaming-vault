#![no_std]

mod types;
#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, token, Address, Env};
use types::{StreamKey, StreamState};

// ── Instance storage keys ─────────────────────────────────────────────────────

const ADMIN: &str = "admin";
const TOKEN: &str = "token";

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct VestingVault;

#[contractimpl]
impl VestingVault {
    /// Initialize the contract with admin and SAC token addresses. Can only be called once.
    pub fn init(env: Env, admin: Address, token: Address) {
        let storage = env.storage().instance();
        if storage.has(&ADMIN) {
            panic!("already initialized");
        }
        storage.set(&ADMIN, &admin);
        storage.set(&TOKEN, &token);
    }

    /// Create a new linear vesting stream for `recipient`. Admin auth required.
    /// Transfers `total_amount` tokens from admin into the contract.
    pub fn create_stream(
        env: Env,
        recipient: Address,
        total_amount: i128,
        start_time: u64,
        end_time: u64,
    ) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        assert!(total_amount > 0, "amount must be positive");
        assert!(end_time > start_time, "end_time must be after start_time");

        let key = StreamKey::Stream(recipient.clone());
        assert!(
            !env.storage().persistent().has(&key),
            "stream already exists"
        );

        let stream = StreamState {
            recipient: recipient.clone(),
            total_amount,
            claimed_amount: 0,
            start_time,
            end_time,
        };
        env.storage().persistent().set(&key, &stream);

        let token_addr: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&admin, env.current_contract_address(), &total_amount);
    }

    /// Return the currently unlocked (claimable) token amount for `recipient`.
    pub fn claimable_amount(env: Env, recipient: Address) -> i128 {
        let key = StreamKey::Stream(recipient.clone());
        let stream: StreamState = env.storage().persistent().get(&key).unwrap();
        Self::unlocked(&env, &stream) - stream.claimed_amount
    }

    /// Withdraw all currently unlocked tokens to `recipient`. Recipient auth required.
    pub fn withdraw(env: Env, recipient: Address) {
        recipient.require_auth();

        let key = StreamKey::Stream(recipient.clone());
        let mut stream: StreamState = env.storage().persistent().get(&key).unwrap();

        let claimable = Self::unlocked(&env, &stream) - stream.claimed_amount;
        assert!(claimable > 0, "nothing to withdraw");

        stream.claimed_amount += claimable;
        env.storage().persistent().set(&key, &stream);

        let token_addr: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&env.current_contract_address(), &recipient, &claimable);
    }

    /// Cancel an active stream, returning all unclaimed tokens to admin. Admin auth required.
    pub fn cancel_stream(env: Env, recipient: Address) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        let key = StreamKey::Stream(recipient.clone());
        let stream: StreamState = env.storage().persistent().get(&key).unwrap();

        let unclaimed = stream.total_amount - stream.claimed_amount;
        env.storage().persistent().remove(&key);

        if unclaimed > 0 {
            let token_addr: Address = env.storage().instance().get(&TOKEN).unwrap();
            let token_client = token::Client::new(&env, &token_addr);
            token_client.transfer(&env.current_contract_address(), &admin, &unclaimed);
        }
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Compute linearly unlocked tokens for `stream` at the current ledger time.
    /// Returns a value in [0, stream.total_amount].
    fn unlocked(env: &Env, stream: &StreamState) -> i128 {
        let now = env.ledger().timestamp();
        if now <= stream.start_time {
            return 0;
        }
        if now >= stream.end_time {
            return stream.total_amount;
        }
        let elapsed = (now - stream.start_time) as i128;
        let duration = (stream.end_time - stream.start_time) as i128;
        stream.total_amount * elapsed / duration
    }
}
