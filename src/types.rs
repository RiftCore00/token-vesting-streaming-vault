use soroban_sdk::{contracttype, Address};

/// Holds the state for a single linear vesting stream.
///
/// Stored in persistent storage keyed by the recipient's address.
#[contracttype]
#[derive(Clone)]
pub struct StreamState {
    /// The beneficiary address that may withdraw unlocked tokens.
    pub recipient: Address,
    /// Total number of tokens allocated to this stream.
    pub total_amount: i128,
    /// Tokens already withdrawn by the recipient.
    pub claimed_amount: i128,
    /// Unix timestamp (seconds) when vesting begins.
    pub start_time: u64,
    /// Unix timestamp (seconds) when the stream is fully vested.
    pub end_time: u64,
}
