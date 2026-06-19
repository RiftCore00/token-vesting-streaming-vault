# Soroban SDK and Network Compatibility

Last reviewed: 2026-06-20.

This contract currently pins `soroban-sdk = 26.1.0` in `Cargo.toml`. Treat the
SDK major version as the protocol-family boundary: build and test with the SDK
line that matches the Stellar network protocol you plan to deploy to.

Primary source: Stellar's software version matrix at
https://developers.stellar.org/docs/networks/software-versions.

## Compatibility Matrix

| Stellar network protocol | Network rollout | Smart Contract Rust SDK | Notes for this repo |
| --- | --- | --- | --- |
| Protocol 27 | Testnet, June 18 2026 | TBD | Do not target protocol 27-only behavior until Stellar publishes the matching Smart Contract Rust SDK and this repo is retested against it. |
| Protocol 26 | Mainnet, May 6 2026; Testnet, April 16 2026 | 26.0.1 in the Stellar matrix; this repo uses the 26.1.x patch line | Current target for this repo. The `26.1.0` SDK pin remains in the protocol 26 family; run the full CI suite after any SDK patch bump. |
| Protocol 25 | Mainnet, January 22 2026; Testnet, January 7 2026 | 25.0.0 | Use only if deploying to infrastructure still pinned to protocol 25. Retest snapshots before downgrading from the current SDK line. |
| Protocol 24 | Mainnet, October 22 2025; Testnet, October 21 2025 | 23.0.3 | Protocol 24 was a stability upgrade after protocol 23 and did not require a new contract SDK major line. |
| Protocol 23 | Mainnet, September 3 2025; Testnet, August 14 2025 | 23.0.2 on mainnet; 23.0.0-rc.2.4 on testnet rollout | Includes unified events and state archival changes. Upgrade from older SDKs before using these network behaviors. |
| Protocol 22 | Mainnet, December 5 2024 | 22.0.3 | Constructor and BLS12-381 host-function support. |
| Protocol 21 | Mainnet, June 18 2024; Testnet, May 20 2024 | 21.0.1-preview.3 | First stable protocol 21 support. Older preview builds were testnet-only. |
| Protocol 20 | Mainnet, March 19 2024 | 20.4.0 | Soroban phase 2. Avoid for new deployments unless you are maintaining a legacy network setup. |

## Upgrade Rules

- Check the Stellar software version matrix before changing `soroban-sdk` in
  `Cargo.toml`.
- Keep `soroban-sdk`, `soroban-sdk` testutils, and the generated `Cargo.lock`
  dependency family aligned.
- Rebuild with `wasm32v1-none`, run `cargo test`, and deploy to testnet before
  any mainnet rollout.
- Do not rely on new protocol host functions until the target network protocol
  is active and the matching SDK line is published.
- If the Stellar matrix marks a Smart Contract Rust SDK as `TBD`, treat that
  protocol as not ready for this contract's SDK upgrade path.
