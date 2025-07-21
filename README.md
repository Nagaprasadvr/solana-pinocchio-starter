# Solana Pinocchio Starter

## Steps to start (Please use wsl or linux for smooth devex)

### 1. clone the repo

```bash
git clone git@github.com:Nagaprasadvr/solana-pinocchio-starter.git
```

### 2. Directory structure

- [src/](src/)

  - [entrypoint.rs](src/entrypoint.rs) - the entrypoint of the program

    - **Note:** it uses nostd_panic_handler to handle panics
      also global allocator is disabled meaning no heap allocations

  - [lib.rs](src/lib.rs) - lib crate

    - **Note:** uses no_std so we cannot use std library (for performance tweaks)

  - [instruction](src/instruction) - all instructions are defined here

  - [state](src/state/) - all account states are defined here

    - [utils.rs](src/state/utils.rs) - utils for state which provide serialization and deserialization helper fns( load_acc , load_mut_acc, etc)

  - [error.rs](program/src/error.rs) - program errors are listed here

- [tests](tests/) - all tests are defined here

  - **Note:** we are using mollusk-svm - a lightweight solana testing framework for running tests in a local environment without the need of a full solana cluster
  - [elfs](tests/elfs/) - compiled solana elfs can be added here and loaded to mollusk while testing
  - [unit_tests.rs](tests/unit_tests.rs) - has the unit tests for the program

- [benches](benches/) - all the benchmarks are defined here
  - [compute_units.md](benches/compute_units.md) - compute unit benchmarks

### 3. Build program

```bash
cargo build-sbf
```

- After build is successful get the program pubkey and replace with the pinocchio_pubkey::declare_id!(...)

```bash
solana address -k target/deploy/solana_pinocchio_starter-keypair.json
```

### 4. Running Tests

```bash
cargo test --features test-default
```

### 5. Running Benchmarks

```bash
cargo bench --features bench-default
```

#### 2025-07-15 08:15:04.827187 UTC

Solana CLI Version: solana-cli 2.2.17 (src:e998175d; feat:3073396398, client:Agave)

| Name              | CUs  | Delta  |
| ----------------- | ---- | ------ |
| InitializeMyState | 3357 | +60    |
| UpdateMyState     | 465  | -1,355 |

### 5. Client Generation

Shank support has been added to generate the client code for the program in ts/rust languages using Codama.

Run this from the root of the repo and change permissions for the bash script if needed (chmod +x gen-client.sh)

```bash
# Generate TypeScript client
./gen-client.sh typescript

# Generate Rust client
./gen-client.sh rust

```

- This will generate the client code in the `client` directory using Codama library
- Idl is generated and stored in the `client/idl` directory
- Generated code structure:
  - TypeScript: [client/ts/generated](client/ts/generated)
  - Rust: [client/rust/generated](client/rust/generated)
  - Each contains:
    - accounts - all the accounts are defined here
    - instructions - all the instructions are defined here
    - types - all the types are defined here
