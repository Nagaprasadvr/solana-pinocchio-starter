#!/usr/bin/env bash
set -e

LANG="$1"

if [[ -z "$LANG" ]]; then
  echo "Usage: $0 <typescript|rust>"
  exit 1
fi

if [[ "$LANG" == "typescript" ]]; then
  cd client
  bun install
  bun run solita
elif [[ "$LANG" == "rust" ]]; then
  if [[ ! -x client/.crates/bin/shank ]]; then
    echo "Installing shank locally..."
    cargo install shank-cli --root client/.crates --version 0.4.2
  fi
  mkdir -p client/idl
  client/.crates/bin/shank idl --crate-root program --out-dir client/idl --out-filename solana_pinocchio_starter.json
  cd client
  bun install
  bun run codama
else
  echo "Unknown language: $LANG"
  exit 1
fi