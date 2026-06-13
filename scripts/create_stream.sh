#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "Usage: $0 --network <network> --source <identity> --contract-id <id> --recipient <address> --amount <i128> --start <unix_ts> --end <unix_ts>"
  exit 1
}

NETWORK="" SOURCE="" CONTRACT_ID="" RECIPIENT="" AMOUNT="" START="" END=""
while [[ $# -gt 0 ]]; do
  case $1 in
    --network)     NETWORK="$2";     shift 2 ;;
    --source)      SOURCE="$2";      shift 2 ;;
    --contract-id) CONTRACT_ID="$2"; shift 2 ;;
    --recipient)   RECIPIENT="$2";   shift 2 ;;
    --amount)      AMOUNT="$2";      shift 2 ;;
    --start)       START="$2";       shift 2 ;;
    --end)         END="$2";         shift 2 ;;
    *) usage ;;
  esac
done
[[ -z "$NETWORK" || -z "$SOURCE" || -z "$CONTRACT_ID" || -z "$RECIPIENT" || -z "$AMOUNT" || -z "$START" || -z "$END" ]] && usage

soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOURCE" \
  --network "$NETWORK" \
  -- create_stream \
  --recipient "$RECIPIENT" \
  --total_amount "$AMOUNT" \
  --start_time "$START" \
  --end_time "$END"
