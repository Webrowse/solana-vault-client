# Solana Vault Client (Rust)

This is a Rust CLI tool to interact with the on-chain Solana Vault Program.  
It sends transactions, passes instructions, and reads on-chain data.  

## Features

- Reads keypairs from local `dirs` location
- Sends custom instruction data to on-chain program
- Uses raw Solana SDK (no Anchor)
- Compatible with devnet deployments

## Directory Structure

```
solana-vault-client/
├── src/
│   └── main.rs         # CLI logic and transaction builder
├── vault_account.json  # Target on-chain account (PDA)
├── Cargo.toml
└── README.md
```

## Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/Webrowse/solana-vault-client.git
cd solana-vault-client
```

### 2. Update Program ID and Vault Address

Edit `src/main.rs` and update:

```rust
let program_id = Pubkey::from_str("YourProgramIDHere").unwrap();
let vault_account = Pubkey::from_str("VaultPDAHere").unwrap();
```

Or load dynamically from `vault_account.json`.

### 3. Run the CLI

```bash
cargo run
```

## Keypair Management

Make sure your wallet keypair is available under:

```bash
~/.config/solana/id.json
```

The program loads this automatically using the `dirs` crate.

## Requirements

- Rust + Cargo
- `solana-cli` installed and configured (`solana config get`)
- On-chain program deployed to devnet or localnet

## Notes

- This is a low-level client using only `solana_sdk`
- Best for debugging, raw testing, and educational usage
- Designed to pair with [`solana-raw`](https://github.com/Webrowse/solana-raw)

## License

MIT
