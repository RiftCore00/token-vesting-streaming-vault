# Threat Model

## Overview

This document describes the trust assumptions, attack surface, and mitigations for the Token Vesting & Streaming Vault smart contract on the Stellar/Soroban platform.

## Trust Assumptions

| Actor | Trusted? | Notes |
|---|---|---|
| Admin address | **Yes** | Controls stream creation. Compromise = funds at risk for unclaimed streams. |
| Recipient address | **Partial** | Can only withdraw their own stream. Cannot access others. |
| SAC token contract | **Yes** | Assumed to be a standard Stellar Asset Contract. Malicious token could reenter. |
| Soroban runtime | **Yes** | We rely on the Soroban VM for execution correctness and auth enforcement. |

## Attack Surface

### 1. Admin Key Compromise
- **Threat**: An attacker who obtains the admin private key can create arbitrary streams, draining tokens allocated to the contract.
- **Impact**: High — all future deposits controlled by admin.
- **Mitigation**: Use a hardware wallet or multi-sig for the admin key. Consider rotating admin.

### 2. Reentrancy via Token Contract
- **Threat**: A malicious SAC token could call back into `withdraw` or `create_stream` during a `transfer`.
- **Impact**: Medium — could allow double-withdrawal.
- **Mitigation**: The contract follows checks-effects-interactions: `claimed_amount` is updated in storage *before* calling `token.transfer`. A reentrant call would see the updated state and find nothing to withdraw.

### 3. Recipient Spoofing
- **Threat**: An attacker calls `withdraw` pretending to be a recipient.
- **Impact**: High — could steal recipient's tokens.
- **Mitigation**: `recipient.require_auth()` enforces Soroban authentication. Only the holder of the recipient's private key can authorize the call.

### 4. Integer Overflow in Vesting Math
- **Threat**: Large `total_amount` values could overflow in `total_amount * elapsed / duration`.
- **Impact**: Medium — incorrect claimable amounts, potential panic.
- **Mitigation**: Soroban `i128` multiplication panics on overflow (overflow-checks enabled). The maximum safe `total_amount` for a 1-second stream is `i128::MAX`. See issue #40 for large-value tests.

### 5. Re-initialization
- **Threat**: An attacker calls `init` again to replace the admin or token address.
- **Impact**: Critical — would redirect all future streams.
- **Mitigation**: `init` checks for an existing `ADMIN` key and panics if already set.

### 6. Duplicate Stream Creation
- **Threat**: Admin accidentally creates a second stream for the same recipient (e.g., fat-finger).
- **Impact**: Low — would panic with "stream already exists", wasting gas.
- **Mitigation**: The duplicate check prevents overwriting; no funds are lost.

### 7. Stream Never Expires
- **Threat**: No mechanism to reclaim tokens from a stream after `end_time` if recipient never withdraws.
- **Impact**: Low — tokens remain locked in the contract indefinitely.
- **Mitigation**: Out of scope for the base contract. A `cancel_stream` function (see issue #11) would address this.

## Out of Scope

- Stellar network-level attacks (eclipse attacks, validator collusion)
- Wallet/client-side security
- Social engineering of the admin
- Token contracts not following the SAC standard

## Risk Matrix

| Threat | Likelihood | Impact | Risk |
|---|---|---|---|
| Admin key compromise | Low | High | **Medium** |
| Reentrancy | Very Low | Medium | **Low** |
| Recipient spoofing | Very Low | High | **Low** |
| Integer overflow | Very Low | Medium | **Low** |
| Re-initialization | Very Low | Critical | **Low** |
| Locked tokens (no cancel) | Medium | Low | **Low** |
