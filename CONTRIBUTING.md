# Contributing to Token Vesting & Linear Streaming Vault

Thank you for your interest in contributing! This document outlines the development workflow, project structure, and guidelines for contributors.

## Project Structure

```
├── src/
│   ├── lib.rs           # Contract entry points & business logic
│   ├── types.rs         # Data structures (StreamState)
│   └── test.rs          # Unit test suite
├── test_snapshots/      # Soroban test snapshots (ledger state assertions)
├── Cargo.toml           # Rust project configuration
└── .github/workflows/   # CI pipeline
```

## Prerequisites

- [Rust](https://rustup.rs) (stable toolchain)
- `wasm32v1-none` target:
  ```bash
  rustup target add wasm32v1-none
  ```
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup) (for deployment)

## Development Setup

```bash
git clone https://github.com/RiftCore00/token-vesting-streaming-vault.git
cd token-vesting-streaming-vault
cargo build
cargo test
```

## Contract Architecture

### Data Model

```rust
StreamState {
    recipient:      Address,
    total_amount:   i128,
    claimed_amount: i128,
    start_time:     u64,
    end_time:       u64,
}
```

Streams are stored in Soroban **persistent** storage keyed by recipient address. Admin and token addresses are stored in **instance** storage.

### Vesting Math

```
unlocked = total_amount * min(elapsed, duration) / duration
```

- Integer division truncates toward zero, always rounding in favor of the contract (never over-pays).
- The `claimable_amount` is `unlocked - claimed_amount`.

## Coding Standards

### Rust Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and address all warnings
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Testing

Every new feature must include tests. The test suite covers:

- **init**: success, double-init panic
- **create_stream**: success, invalid time range, zero amount, duplicate stream panic
- **claimable_amount**: before start, at midpoint, after end, after partial withdrawal
- **withdraw**: partial, full, empty (panic), multiple sequential withdrawals

To add test snapshots for new scenarios:

```rust
#[test]
fn test_my_new_feature() {
    let env = Env::default();
    let (client, admin, recipient, token_id) = setup(&env);
    // ... test logic ...
}
```

Run tests with:

```bash
cargo test
```

## Security Considerations

- **Checks-effects-interactions**: State is updated before external token transfers.
- **Integer overflow**: Compile-time checked in release builds (`overflow-checks = true`).
- **Re-initialization**: Guarded by an explicit storage check.
- **Authentication**: Admin-gated stream creation, recipient-gated withdrawals.

## Pull Request Process

1. Fork the repository and create a feature branch from `master`.
2. Make your changes, ensuring tests pass (`cargo test`).
3. Run `cargo fmt` and `cargo clippy`.
4. Open a pull request with a clear description of the change and link to any related issues.
5. Ensure CI passes (test + WASM build).

## Good First Issues

Look for issues tagged `good-first-issue` in the repository. These typically involve:

- Additional test coverage for edge cases
- Documentation improvements
- Helper functions for common operations

## Questions?

Open a discussion in the GitHub repository or reach out via issue comments. All communication happens in the open.
