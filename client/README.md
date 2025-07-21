# client

To install dependencies:

```bash
bun install
```

To generate the client:

```bash
# Generate TypeScript client
bun run gen-client typescript

# Generate Rust client
bun run gen-client rust

# Format Rust client
rustfmt rust/generated/*.rs rust/generated/**/*.rs
```

This will generate client code using Codama in:

- TypeScript: `ts/generated/`
- Rust: `rust/generated/` (auto-formatted)
