# Mainnet Deployment Guide

This guide walks through deploying the Token Vesting & Streaming Vault contract to Stellar **mainnet**. Always test on testnet first (see [TESTNET.md](TESTNET.md)).

## Prerequisites

- **Rust** with `wasm32v1-none` target:
  ```bash
  rustup target add wasm32v1-none
  ```
- **Soroban CLI** (latest stable):
  ```bash
  cargo install --locked soroban-cli
  ```
- A **funded mainnet account** (admin). You need XLM to pay transaction fees and the contract's minimum balance.
- The **SAC token address** you want to use for streams (must be a Stellar Asset Contract).
- Confirm the pinned `soroban-sdk` line matches the target network protocol.
  See [SOROBAN_COMPATIBILITY.md](SOROBAN_COMPATIBILITY.md).

## Safety Checklist Before Deploying to Mainnet

- [ ] Contract fully tested on testnet with your specific token
- [ ] Admin key is secure (hardware wallet or multisig recommended)
- [ ] You have verified the SAC token address is correct
- [ ] You understand the contract is **not upgradeable** — deploy carefully
- [ ] Contract has been audited or reviewed for your use case

## Step 1: Build the Contract

```bash
cargo build --target wasm32v1-none --release
```

The output WASM will be at:
```
target/wasm32v1-none/release/token_vesting_streaming_vault.wasm
```

Verify the build succeeded:
```bash
ls -lh target/wasm32v1-none/release/token_vesting_streaming_vault.wasm
```

## Step 2: Configure Soroban CLI for Mainnet

Add your admin identity (or import an existing key):
```bash
soroban keys generate --global admin
# or import existing:
soroban keys add --global admin --secret-key
```

Configure the mainnet network:
```bash
soroban network add --global mainnet \
  --rpc-url https://soroban-rpc.mainnet.stellar.gateway.fm \
  --network-passphrase "Public Global Stellar Network ; September 2015"
```

## Step 3: Deploy the Contract

```bash
soroban contract deploy \
  --wasm target/wasm32v1-none/release/token_vesting_streaming_vault.wasm \
  --source admin \
  --network mainnet
```

Save the output `CONTRACT_ID`.

## Step 4: Initialize the Contract

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  --network mainnet \
  -- \
  init \
  --admin <ADMIN_ADDRESS> \
  --token <SAC_TOKEN_ADDRESS>
```

> **Note:** `<ADMIN_ADDRESS>` is the Stellar G-address of the admin account. `<SAC_TOKEN_ADDRESS>` is the contract ID of the Stellar Asset Contract for your token.

## Step 5: Post-Deployment Verification

Verify the contract is initialized by creating a test query (should return 0 for an unknown address or panic with "no stream"):

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network mainnet \
  -- \
  claimable_amount \
  --recipient <SOME_ADDRESS>
```

## Step 6: Create Your First Stream

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  --network mainnet \
  -- \
  create_stream \
  --recipient <RECIPIENT_ADDRESS> \
  --total_amount 1000000000 \
  --start_time 1700000000 \
  --end_time 1731536000
```

> Timestamps are Unix seconds. `total_amount` is in the token's base unit (check the token's decimal places).

## Finding the SAC Token Address

For a Stellar classic asset (e.g., USDC issued by `GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN`):

```bash
soroban contract id asset \
  --asset USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN \
  --network mainnet
```

See [ASSET_WRAPPING.md](ASSET_WRAPPING.md) for wrapping classic assets into SAC tokens.

## Troubleshooting

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common errors.
