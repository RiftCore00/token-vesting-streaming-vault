# Soroban SDK Upgrade Checklist

Use this checklist when changing the `soroban-sdk` version in `Cargo.toml`.
SDK upgrades affect contract serialization, host behavior, generated WASM, and
test snapshots, so keep dependency, build, test, and deployment checks together
in one reviewable change.

## 1. Confirm The Target Version

- Read the Soroban SDK release notes for breaking API, host, auth, event, and
  storage changes.
- Confirm the Rust channel in `rust-toolchain.toml` still supports the target
  SDK version.
- Confirm the WASM target remains installed:

  ```bash
  rustup target add wasm32v1-none
  ```

## 2. Update Dependencies

Update both `soroban-sdk` entries in `Cargo.toml`:

```toml
[dependencies]
soroban-sdk = { version = "<new-version>", features = ["alloc"] }

[dev-dependencies]
soroban-sdk = { version = "<new-version>", features = ["testutils", "alloc"] }
```

Then refresh the lockfile intentionally:

```bash
cargo update -p soroban-sdk
```

Review the `Cargo.lock` diff for transitive dependency changes. Do not mix
unrelated dependency updates into the SDK upgrade PR.

## 3. Run Host Tests And Refresh Snapshots

Run the test suite:

```bash
cargo test
```

If Soroban ledger behavior, event output, or storage encoding changed, refresh
the affected files under `test_snapshots/` and commit those snapshot changes
with the SDK upgrade.

When a snapshot changes, record why it changed in the PR summary. Treat
unexpected storage-key, auth, event, or balance differences as possible behavior
changes, not formatting noise.

## 4. Run Formatting And Lints

Match the repository CI jobs before opening the PR:

```bash
cargo fmt --check
cargo clippy -- -D warnings
```

Address new lints directly. Avoid broad `allow` attributes unless the warning is
known to be false-positive and the reason is documented near the code.

## 5. Build The Release WASM

Build the contract with the same target used by CI and deployment docs:

```bash
cargo build --target wasm32v1-none --release
```

Confirm the expected artifact exists:

```bash
ls -lh target/wasm32v1-none/release/token_vesting_streaming_vault.wasm
```

If the artifact name, target directory, or build flags change, update
`README.md`, `docs/DEPLOYMENT.md`, `scripts/README.md`, and CI in the same PR.

## 6. Check Public Contract Compatibility

Review the public contract surface after the SDK bump:

- entrypoint names and parameters in `src/lib.rs`
- `StreamState` serialization in `src/types.rs`
- auth requirements for `init`, `create_stream`, and `withdraw`
- event topics and payloads, if changed by the SDK
- panic/error behavior that callers may observe

Document any intentional compatibility break in the PR body and update
integration docs before merging.

## 7. Verify Deployment Commands

After the release WASM build succeeds, review the deployment flow in
`docs/DEPLOYMENT.md`:

```bash
soroban contract deploy \
  --wasm target/wasm32v1-none/release/token_vesting_streaming_vault.wasm \
  --source <identity> \
  --network <network>
```

If the SDK upgrade requires a newer Soroban CLI, update the prerequisite text in
`README.md`, `CONTRIBUTING.md`, `docs/DEPLOYMENT.md`, and `scripts/README.md`.

## 8. PR Checklist

Before requesting review, confirm the PR includes:

- `Cargo.toml` SDK version updates
- intentional `Cargo.lock` changes
- refreshed `test_snapshots/` files when ledger output changed
- documentation updates for any changed commands, artifact paths, or CLI
  requirements
- a summary of compatibility impact for existing deployed contracts and
  integrators

CI must pass the repository jobs for formatting, clippy, audit, tests, and the
release WASM build before the upgrade is ready to merge.
