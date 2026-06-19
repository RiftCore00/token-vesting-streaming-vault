# Troubleshooting Common Build Errors

Use this guide when `cargo test`, `cargo build --target wasm32v1-none --release`,
or the Soroban CLI deployment flow fails during local development.

## Quick Environment Check

Start by confirming the toolchain and target that this repository expects:

```bash
rustup show active-toolchain
rustup target list --installed | grep wasm32v1-none
cargo --version
```

The repository pins the stable Rust channel and the `wasm32v1-none` target in
`rust-toolchain.toml`. If the target is missing, install it with:

```bash
rustup target add wasm32v1-none
```

## Missing `wasm32v1-none` Target

Symptom:

```text
error[E0463]: can't find crate for `core`
```

or:

```text
error: target `wasm32v1-none` not found
```

Fix:

```bash
rustup update stable
rustup target add wasm32v1-none
cargo build --target wasm32v1-none --release
```

If `rustup target add wasm32v1-none` still fails, make sure the active toolchain
is stable and that `rust-toolchain.toml` is present in the repository root.

## Linker Or Native Toolchain Errors

Symptom:

```text
linker `cc` not found
```

or host-specific linker failures while building dependencies.

Fix:

1. Install the platform C toolchain required by your OS.
2. Re-run the host checks first:

   ```bash
   cargo test
   cargo clippy -- -D warnings
   ```

3. Re-run the WASM build:

   ```bash
   cargo build --target wasm32v1-none --release
   ```

The contract itself builds to WASM, but some Rust crates still compile host
build scripts or test utilities that need a working native toolchain.

## Soroban SDK Or Lockfile Drift

Symptom:

```text
the lock file needs to be updated but --locked was passed
```

or dependency resolution changes after editing `Cargo.toml`.

Fix:

```bash
cargo update -p soroban-sdk
cargo test
cargo build --target wasm32v1-none --release
```

Only commit `Cargo.lock` changes when the dependency update is intentional. The
contract currently uses the `soroban-sdk` version declared in `Cargo.toml`.

## Clippy Fails On Warnings

Symptom:

```text
error: could not compile ... due to previous warning
```

The CI runs:

```bash
cargo clippy -- -D warnings
```

Fix the warning rather than suppressing it. For documentation-only changes, this
still runs in CI because the repository validates all pull requests.

## Formatting Fails

Symptom:

```text
Diff in ...
```

CI checks Rust formatting with:

```bash
cargo fmt --check
```

Fix:

```bash
cargo fmt
cargo fmt --check
```

## WASM Artifact Not Found During Deployment

Symptom:

```text
No such file or directory: target/wasm32v1-none/release/token_vesting_streaming_vault.wasm
```

Fix:

```bash
cargo build --target wasm32v1-none --release
ls -lh target/wasm32v1-none/release/token_vesting_streaming_vault.wasm
```

Run the deploy command only after the release WASM exists.

## Soroban CLI Command Not Found

Symptom:

```text
soroban: command not found
```

Fix:

```bash
cargo install --locked soroban-cli
soroban --version
```

Then follow `scripts/README.md` or `docs/DEPLOYMENT.md` for network, identity,
and contract invocation commands.

## When CI Fails But Local Build Passes

Check the failing job name first:

- `fmt`: run `cargo fmt --check`
- `clippy`: run `cargo clippy -- -D warnings`
- `audit`: inspect the `cargo audit` advisory and update only affected
  dependencies intentionally
- `test-and-build`: run `cargo test`, then
  `cargo build --target wasm32v1-none --release`

If a test changes contract storage behavior, refresh the relevant
`test_snapshots/` files with `cargo test` and commit the updated snapshots with
the code change.
