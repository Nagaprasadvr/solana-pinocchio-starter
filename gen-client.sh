set -e

LANG="$1"

if [[ -z "$LANG" ]]; then
  echo "Usage: $0 <typescript|rust|all>"
  exit 1
fi

# Generate IDL first
if [[ ! -x client/.crates/bin/shank ]]; then
  echo "Installing shank locally..."
  cargo install shank-cli --root client/.crates --version 0.4.2
fi

mkdir -p client/idl
client/.crates/bin/shank idl --crate-root program --out-dir client/idl --out-filename solana_pinocchio_starter.json

# Generate client
cd client
bun install
bun run gen-client "$LANG"

if [[ "$LANG" == "rust" || "$LANG" == "all" ]]; then
  rustfmt rust/generated/*.rs rust/generated/**/*.rs
fi