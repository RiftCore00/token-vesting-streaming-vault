# Token Vesting & Linear Streaming Vault

A native Soroban smart contract designed to lock up Stellar Asset Contract (SAC) tokens and stream them to beneficiaries linearly over time based on on-chain ledger timestamps.

## Architecture & Layout

```
├── src/
│   ├── lib.rs          # Contract entry point and external interface
│   ├── types.rs        # Custom data structures (StreamState, etc.)
│   └── test.rs         # Execution test suite
├── Cargo.toml          # Rust dependencies & Soroban SDK configuration
└── README.md
```

## Core Functions

- **`init(env, admin: Address, token: Address)`** — Initializes the contract configuration.
- **`create_stream(env, recipient: Address, total_amount: i128, start_time: u64, end_time: u64)`** — Allocates tokens to a linear stream.
- **`claimable_amount(env, recipient: Address) -> i128`** — Read-only view of unlocked tokens.
- **`withdraw(env, recipient: Address)`** — Transfers unlocked tokens to the beneficiary.

## Implementation Roadmap

### Issue #1 — Define Data Structures & Init
Implement `types.rs` containing the `StreamState` struct and the basic `init` code in `lib.rs`. Store the token address and admin state using Soroban Instance storage.

### Issue #2 — Implement Linear Math View
Build the `claimable_amount` math helper. Fetch `env.ledger().timestamp()` and calculate unlocked token proportions without floating-point math to prevent precision loss.

### Issue #3 — Secure Withdrawal Logic
Write the state-updating `withdraw` function. Enforce `recipient.auth()`, pull tokens from contract storage via `token::Client`, and accurately update internal balances to avoid reentrancy vectors.

## How to Build & Test

```bash
soroban contract build
cargo test
```
