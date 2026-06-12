# Token Vesting & Linear Streaming Vault

A Soroban smart contract for time-based linear vesting and streaming of Stellar Asset Contract (SAC) tokens. Locks tokens and releases them proportionally over a specified time window, allowing beneficiaries to withdraw unlocked tokens at any point.

## Features

- Linear token streaming with integer-only math
- Per-recipient isolated streams
- Admin-gated stream creation
- Recipient-authenticated withdrawals
- Idempotent incremental withdrawals
