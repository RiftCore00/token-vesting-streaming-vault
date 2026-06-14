#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "Usage: $0 --network <network> --source <identity> --contract-id <id> --admin <address> --token <address>"
  exit 1
}

NETWORK="" SOURCE="" CONTRACT_ID="" ADMIN="" TOKEN=""
while [[ $# -gt 0 ]]; do
  case $1 in
    --network)     NETWORK="$2";      shift 2 ;;
    --source)      SOURCE="$2";       shift 2 ;;
    --contract-id) CONTRACT_ID="$2";  shift 2 ;;
    --admin)       ADMIN="$2";        shift 2 ;;
    --token)       TOKEN="$2";        shift 2 ;;
    *) usage ;;
  esac
done
[[ -z "$NETWORK" || -z "$SOURCE" || -z "$CONTRACT_ID" || -z "$ADMIN" || -z "$TOKEN" ]] && usage

soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOURCE" \
  --network "$NETWORK" \
  -- init \
  --admin "$ADMIN" \
  --token "$TOKEN"
