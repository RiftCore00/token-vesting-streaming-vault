# Token Vesting & Linear Streaming Vault

A [Soroban](https://soroban.stellar.org) smart contract for time-based linear vesting and streaming of [Stellar Asset Contract (SAC)](https://developers.stellar.org/docs/smart-contracts/tokens) tokens. The contract locks tokens and releases them proportionally over a specified time window, allowing beneficiaries to withdraw unlocked tokens at any point.

## Features

- **Linear token streaming** — Tokens unlock continuously at a constant rate between `start_time` and `end_time`.
- **Integer-only math** — All vesting calculations use integer arithmetic, avoiding floating-point precision loss.
- **Re-initialization protection** — Contract can be initialized exactly once.
- **Per-recipient isolation** — Each recipient has an independent stream stored in persistent storage.
- **Admin-gated stream creation** — Only the admin address can create new vesting streams.
- **Recipient-authenticated withdrawals** — Recipients must authenticate to withdraw their unlocked tokens.
- **Idempotent withdrawals** — Claimed amount is tracked, ensuring recipients can withdraw incrementally without over-drawing.

## Architecture

```
├── .github/workflows/ci.yml   # CI pipeline (test + wasm build)
├── src/
│   ├── lib.rs                  # Contract entry points & business logic
│   ├── types.rs                # Data structures (StreamState)
│   └── test.rs                 # Unit test suite
├── Cargo.toml                  # Rust project configuration
└── README.md
```

### Data Model

```
StreamState {
    recipient:      Address,      // Beneficiary address
    total_amount:   i128,         // Total tokens allocated
    claimed_amount: i128,         // Tokens already withdrawn
    start_time:     u64,          // Unix timestamp (seconds) when vesting begins
    end_time:       u64,          // Unix timestamp (seconds) when fully vested
}
```

Streams are stored in Soroban **persistent** storage keyed by recipient address. Admin and token addresses are stored in **instance** storage.

## Contract Interface

| Function | Auth | Parameters | Description |
|---|---|---|---|
| `init` | — | `admin: Address`, `token: Address` | One-time initialization. Sets the contract admin and the SAC token address. |
| `create_stream` | `admin` | `recipient: Address`, `total_amount: i128`, `start_time: u64`, `end_time: u64` | Creates a linear vesting stream. Transfers `total_amount` tokens from admin into the contract. |
| `claimable_amount` | — (read-only) | `recipient: Address` → `i128` | Returns the number of unlocked tokens not yet withdrawn. |
| `withdraw` | `recipient` | `recipient: Address` | Transfers all currently unlocked tokens to the recipient. |

### Vesting Math

Given a stream with `total_amount` tokens unlocking linearly from `start_time` to `end_time`:

```
unlocked = total_amount * min(elapsed, duration) / duration

where:
  elapsed  = max(0, now - start_time)
  duration = end_time - start_time
```

Division is integer truncation, which is safe because it always rounds in favor of the contract (never over-pays).

## Security Considerations

- **Integer division rounding** — Truncation rounds down, preventing overpayment at the cost of negligible underpayment (≤ 1 unit per withdrawal).
- **Reentrancy** — The contract follows a checks-effects-interactions pattern: state is updated before the external token transfer.
- **Single-stream per recipient** — Creating a second stream for the same address will panic. This prevents ambiguity in `claimable_amount` calculations.
- **Admin authority** — The admin address controls stream creation. No other address can allocate tokens.
- **No `init` reentrancy** — Re-initialization is blocked by an explicit guard.

## Prerequisites

- [Rust](https://rustup.rs) (stable toolchain)
- `wasm32v1-none` target:
  ```bash
  rustup target add wasm32v1-none
  ```
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup) (for deployment)

When changing the Soroban SDK version, follow
[`docs/SDK_UPGRADE_CHECKLIST.md`](docs/SDK_UPGRADE_CHECKLIST.md).

## Build

```bash
cargo build --target wasm32v1-none --release
```

The compiled WASM contract will be at `target/wasm32v1-none/release/token_vesting_streaming_vault.wasm`.

SDK upgrades can change generated WASM and test snapshots. Review
[`docs/SDK_UPGRADE_CHECKLIST.md`](docs/SDK_UPGRADE_CHECKLIST.md) before bumping
`soroban-sdk`.

## Test

```bash
cargo test
```

The test suite covers:

- Contract initialization (success, double-init panic)
- Stream creation (success, invalid time range, zero amount)
- Claimable amount computation (before start, midpoint, after end)
- Withdrawals (partial, full, empty, multiple sequential)

## Deployment

```bash
soroban contract deploy \
  --wasm target/wasm32v1-none/release/token_vesting_streaming_vault.wasm \
  --source <identity> \
  --network <network>
```

After deployment, call `init`:

```bash
soroban contract invoke \
  --id <contract_id> \
  --source <identity> \
  --network <network> \
  -- \
  init \
  --admin <admin_address> \
  --token <sac_token_address>
```

## CI/CD

The project includes a GitHub Actions workflow (`.github/workflows/ci.yml`) that runs on every push and pull request:

1. Installs Rust stable + `wasm32v1-none` target
2. Runs `cargo test`
3. Builds the release WASM binary

## License

This project is open source. See the LICENSE file for details.
