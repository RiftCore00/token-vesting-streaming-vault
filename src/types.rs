use soroban_sdk::{contracttype, Address};

/// Typed storage key for persistent stream records.
/// Keyed by the recipient address to ensure one stream per recipient.
#[contracttype]
#[derive(Clone)]
pub enum StreamKey {
    Stream(Address),
}

/// Represents the state of a single linear vesting stream.
#[contracttype]
#[derive(Clone)]
pub struct StreamState {
    /// The beneficiary address that can withdraw unlocked tokens.
    pub recipient: Address,
    /// Total tokens allocated to this stream.
    pub total_amount: i128,
    /// Tokens already withdrawn by the recipient.
    pub claimed_amount: i128,
    /// Unix timestamp (seconds) when vesting begins.
    pub start_time: u64,
    /// Unix timestamp (seconds) when the stream is fully vested.
    pub end_time: u64,
}
