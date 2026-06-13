#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "Usage: $0 --network <network> --source <identity>"
  exit 1
}

NETWORK="" SOURCE=""
while [[ $# -gt 0 ]]; do
  case $1 in
    --network) NETWORK="$2"; shift 2 ;;
    --source)  SOURCE="$2";  shift 2 ;;
    *) usage ;;
  esac
done
[[ -z "$NETWORK" || -z "$SOURCE" ]] && usage

WASM="target/wasm32v1-none/release/token_vesting_streaming_vault.wasm"
[[ ! -f "$WASM" ]] && { echo "Build first: cargo build --target wasm32v1-none --release"; exit 1; }

soroban contract deploy \
  --wasm "$WASM" \
  --source "$SOURCE" \
  --network "$NETWORK"
