# Scripts

Helper scripts for interacting with the deployed vault contract via the Soroban CLI.

## Prerequisites

- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup) installed
- A configured identity (`soroban keys generate --global <name>`)
- A configured network (`soroban network add --global testnet ...`)

## Scripts

### `deploy.sh`
Deploy the compiled WASM to a network. Outputs the contract ID.
```bash
./scripts/deploy.sh --network testnet --source admin
```

### `init.sh`
Initialize a freshly deployed contract.
```bash
./scripts/init.sh \
  --network testnet \
  --source admin \
  --contract-id <CONTRACT_ID> \
  --admin <ADMIN_ADDRESS> \
  --token <SAC_TOKEN_ADDRESS>
```

### `create_stream.sh`
Create a new vesting stream for a recipient.
```bash
./scripts/create_stream.sh \
  --network testnet \
  --source admin \
  --contract-id <CONTRACT_ID> \
  --recipient <RECIPIENT_ADDRESS> \
  --amount 1000000000 \
  --start 1700000000 \
  --end 1731536000
```

### `withdraw.sh`
Withdraw all claimable tokens for a recipient (must be signed by recipient).
```bash
./scripts/withdraw.sh \
  --network testnet \
  --source recipient \
  --contract-id <CONTRACT_ID> \
  --recipient <RECIPIENT_ADDRESS>
```

### `query.sh`
Query how many tokens are currently claimable for a recipient (read-only).
```bash
./scripts/query.sh \
  --network testnet \
  --contract-id <CONTRACT_ID> \
  --recipient <RECIPIENT_ADDRESS>
```
