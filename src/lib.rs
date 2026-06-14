#![no_std]

//! A Soroban contract for linear token vesting and streaming.
//!
//! The admin creates vesting streams that linearly unlock tokens over a
//! specified time window. Recipients can withdraw their unlocked tokens
//! at any point during or after the vesting period.

mod types;
#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, token, Address, Env};
use types::StreamState;

// Storage keys
const ADMIN: &str = "admin";
const TOKEN: &str = "token";

fn stream_key(recipient: &Address) -> Address {
    recipient.clone()
}

/// The Vesting & Streaming Vault contract.
///
/// Manages linear vesting streams identified by recipient address.
#[contract]
pub struct VestingVault;

#[contractimpl]
impl VestingVault {
    /// Initialize the contract with an admin address and token address.
///
/// # Panics
/// Panics if the contract has already been initialized.
    pub fn init(env: Env, admin: Address, token: Address) {
        let storage = env.storage().instance();
        // Prevent re-initialization
        if storage.has(&ADMIN) {
            panic!("already initialized");
        }
        storage.set(&ADMIN, &admin);
        storage.set(&TOKEN, &token);
    }

    /// Create a new linear vesting stream for a recipient.
    ///
    /// Transfers `total_amount` tokens from the admin into the contract.
    ///
    /// # Panics
    /// - If the caller is not the admin.
    /// - If `total_amount` is not positive.
    /// - If `end_time` is not after `start_time`.
    /// - If a stream for the recipient already exists.
    pub fn create_stream(
        env: Env,
        recipient: Address,
        total_amount: i128,
        start_time: u64,
        end_time: u64,
    ) {
        // Only admin can create streams
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        assert!(total_amount > 0, "amount must be positive");
        assert!(end_time > start_time, "end_time must be after start_time");

        let key = stream_key(&recipient);
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

        // Transfer tokens from admin into the contract
        let token_addr: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&admin, env.current_contract_address(), &total_amount);
    }

    /// Return the amount of tokens currently unlocked and not yet withdrawn for a recipient.
    ///
    /// # Panics
    /// Panics if no stream exists for the recipient.
    pub fn claimable_amount(env: Env, recipient: Address) -> i128 {
        let key = stream_key(&recipient);
        let stream: StreamState = env.storage().persistent().get(&key).unwrap();
        Self::unlocked(&env, &stream) - stream.claimed_amount
    }

    /// Withdraw all currently unlocked tokens to the recipient.
    ///
    /// # Panics
    /// - If the caller is not the recipient.
    /// - If there are no unlocked tokens to withdraw.
    pub fn withdraw(env: Env, recipient: Address) {
        recipient.require_auth();

        let key = stream_key(&recipient);
        let mut stream: StreamState = env.storage().persistent().get(&key).unwrap();

        let claimable = Self::unlocked(&env, &stream) - stream.claimed_amount;
        assert!(claimable > 0, "nothing to withdraw");

        stream.claimed_amount += claimable;
        env.storage().persistent().set(&key, &stream);

        let token_addr: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&env.current_contract_address(), &recipient, &claimable);
    }

    // ── Internal helper ──────────────────────────────────────────────────────

    /// Compute the linearly-unlocked amount for a stream at the current ledger timestamp.
///
/// Returns `0` before `start_time`, `total_amount` after `end_time`, and a
/// proportional fraction (integer division, floored) in between.
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
