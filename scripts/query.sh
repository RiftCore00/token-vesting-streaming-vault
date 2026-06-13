#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "Usage: $0 --network <network> --contract-id <id> --recipient <address>"
  exit 1
}

NETWORK="" CONTRACT_ID="" RECIPIENT=""
while [[ $# -gt 0 ]]; do
  case $1 in
    --network)     NETWORK="$2";     shift 2 ;;
    --contract-id) CONTRACT_ID="$2"; shift 2 ;;
    --recipient)   RECIPIENT="$2";   shift 2 ;;
    *) usage ;;
  esac
done
[[ -z "$NETWORK" || -z "$CONTRACT_ID" || -z "$RECIPIENT" ]] && usage

soroban contract invoke \
  --id "$CONTRACT_ID" \
  --network "$NETWORK" \
  -- claimable_amount \
  --recipient "$RECIPIENT"
