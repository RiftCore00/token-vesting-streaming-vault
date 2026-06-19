# GrantFox Integration Guide

This guide describes how the Token Vesting & Linear Streaming Vault can support
GrantFox milestone-based funding payouts. It maps GrantFox payout operations to
the vault's existing Soroban contract calls; it does not add a separate
GrantFox-specific contract interface.

## Integration Roles

| Role | Vault responsibility |
|---|---|
| GrantFox payout operator | Holds the vault admin key and creates streams after a milestone is approved. |
| Contributor or grantee | Receives the stream and signs `withdraw` calls from the recipient address. |
| SAC token contract | Defines the asset used for payouts and handles token transfers. |
| Vault contract | Locks approved payout tokens and releases them linearly between `start_time` and `end_time`. |

The admin key is the main trust boundary. Use the same operational controls
recommended in [THREAT_MODEL.md](THREAT_MODEL.md): hardware wallet or multisig
custody, verified token addresses, and testnet rehearsal before mainnet use.

## Milestone Payout Lifecycle

1. **Approve the milestone off-chain.**
   GrantFox or the project maintainer approves a contribution milestone and
   determines the recipient address, payout asset, total amount, start time,
   and end time.

2. **Deploy and initialize the vault.**
   Deploy the WASM, then call `init(admin, token)` once with the payout
   operator as `admin` and the selected Stellar Asset Contract as `token`.
   See [DEPLOYMENT.md](DEPLOYMENT.md) and [scripts/README.md](../scripts/README.md).

3. **Create the milestone stream.**
   The admin calls `create_stream(recipient, total_amount, start_time, end_time)`.
   The vault transfers `total_amount` from the admin into the contract and
   stores one persistent stream keyed by `recipient`.

4. **Track vesting progress.**
   GrantFox status pages, maintainers, or recipients can call
   `claimable_amount(recipient)` without recipient authentication to show the
   currently unlocked amount.

5. **Withdraw unlocked funds.**
   The contributor calls `withdraw(recipient)` from the recipient address. The
   vault updates `claimed_amount` before transferring tokens, so repeated
   withdrawals only release newly unlocked funds.

## Command Mapping

The helper scripts in `scripts/` can be used directly in a GrantFox payout runbook.

| GrantFox action | Vault command |
|---|---|
| Deploy payout vault | `./scripts/deploy.sh --network <network> --source <admin_identity>` |
| Bind admin and payout asset | `./scripts/init.sh --network <network> --source <admin_identity> --contract-id <CONTRACT_ID> --admin <ADMIN_ADDRESS> --token <SAC_TOKEN_ADDRESS>` |
| Fund an approved milestone | `./scripts/create_stream.sh --network <network> --source <admin_identity> --contract-id <CONTRACT_ID> --recipient <RECIPIENT_ADDRESS> --amount <BASE_UNITS> --start <UNIX_SECONDS> --end <UNIX_SECONDS>` |
| Show unlocked amount | `./scripts/query.sh --network <network> --contract-id <CONTRACT_ID> --recipient <RECIPIENT_ADDRESS>` |
| Recipient withdrawal | `./scripts/withdraw.sh --network <network> --source <recipient_identity> --contract-id <CONTRACT_ID> --recipient <RECIPIENT_ADDRESS>` |

`total_amount` must use the token's base unit. For example, a token with seven
decimals represents `100.0000000` tokens as `1000000000`.

## Verification Checklist

Before creating a GrantFox payout stream:

- Confirm the milestone approval record matches the `recipient`, amount, asset,
  `start_time`, and `end_time` used in `create_stream`.
- Confirm the vault was initialized with the intended SAC token contract.
- Confirm the admin account has enough token balance and can authorize the
  `create_stream` transfer.
- Use Unix timestamps in seconds and verify `end_time > start_time`.
- Run a testnet dry run for new payout assets or new operator accounts.

After creating a stream:

- Query `claimable_amount` before the start time and confirm it is `0`.
- Query near the midpoint and verify the linear unlock curve matches the
  expected `total_amount * elapsed / duration` calculation.
- Confirm the recipient can withdraw only from the recipient address.
- Keep the contract ID, recipient address, SAC token address, amount, and stream
  window in the GrantFox payout record.

## Current Contract Limits

- The vault supports one active stream per recipient address. Multiple GrantFox
  milestones for the same contributor require separate recipient addresses or a
  future contract extension.
- Streams cannot be cancelled or reclaimed after creation. Create streams only
  after the milestone and payout parameters are final.
- Vesting is linear only. Cliff schedules or milestone-by-milestone unlocks must
  be modeled as separate streams or added in a future contract version.
- `claimable_amount` is read-only, but `withdraw` requires recipient
  authentication and transfers all currently unlocked tokens.

These limits are part of the current contract behavior documented in
[ARCHITECTURE.md](ARCHITECTURE.md) and should be included in any GrantFox payout
operator checklist.
