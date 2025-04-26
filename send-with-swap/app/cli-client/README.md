# ☕️ Send-with-Swap CLI

> A powerful CLI for swapping and sending tokens on Solana, made with 💜 by [Teapot in Wonderland](https://github.com/0x7EAB07)

⚠️ **IMPORTANT: THIS TOOL IS CURRENTLY A WORK IN PROGRESS AND DOES NOT FUNCTION YET. USE AT YOUR OWN RISK!** ⚠️

This command-line interface leverages Jupiter's swap aggregator to send tokens to another account after swapping them, all in a single transaction.

## ✨ Features

- 💰 Initialize a treasury account for fee collection
- 💱 Swap tokens using Jupiter's swap aggregator while sending them to another wallet
- 🔄 Support for SOL, USDC, and more tokens (easily extensible)
- 🔁 Reliable transaction handling with automatic retries
- 🛠️ Modern CLI interface with proper argument handling and helpful documentation

## 🏗️ Development Status

This CLI is currently in active development and **not yet functional**. Major parts of the implementation are still being built, including:

- Integration with the on-chain program
- Proper error handling and recovery
- Final testing with real transactions

Please check back later for a working version, or follow the development on GitHub.

## 🚀 Installation

1. Make sure you have Rust and Cargo installed. If not, get them from [rustup.rs](https://rustup.rs/).

2. Clone the repository and build the CLI:

```bash
cd send-with-swap/app/cli-client
cargo build --release
```

3. The binary will be available at `target/release/send-with-swap-cli`.

## 🔧 Dependencies

For convenience, this project uses [Just](https://github.com/casey/just) as a command runner. If you don't have it installed:

```bash
# On macOS
brew install just

# On Ubuntu
apt install just

# Using cargo
cargo install just
```

## 🚗 Usage

For help with all available commands and options:

```bash
./send-with-swap-cli --help
```

Each subcommand also has detailed help information:

```bash
./send-with-swap-cli init-treasury --help
./send-with-swap-cli send --help
```

### Initialize Treasury

Before using the send-with-swap functionality, you need to initialize a treasury account where fees will be collected:

```bash
# Directly using the CLI
./send-with-swap-cli init-treasury --keypair <KEYPAIR_PATH>

# Or using Just
just init-treasury ~/.config/solana/id.json
```

This will create a new treasury account and print its public key.

### Send with Swap

To send tokens to another wallet while swapping them:

```bash
# Directly using the CLI
./send-with-swap-cli send \
  --keypair <KEYPAIR_PATH> \
  --input-token <INPUT_TOKEN> \
  --output-token <OUTPUT_TOKEN> \
  --amount <AMOUNT> \
  --recipient <RECIPIENT_ADDRESS> \
  [--treasury <TREASURY_ADDRESS>] \
  [--slippage-bps <SLIPPAGE>]

# Or using Just (much simpler!)
just send ~/.config/solana/id.json SOL USDC 0.1 9drLZh2B6oLNTJMKCcEF2VWfPRcT9HoAYTCcPpZ1h4xV
```

This command will:
1. Query Jupiter's swap API for a quote to swap your tokens
2. Execute the swap transaction
3. Send the resulting tokens to the recipient
4. Collect a small fee in the treasury account

If you don't specify a treasury public key, the CLI will find it automatically based on the program ID.

### Using a Custom RPC URL

You can specify a custom RPC URL in two ways:

1. Using the `--rpc-url` flag:
```bash
./send-with-swap-cli --rpc-url https://api.devnet.solana.com send [OPTIONS]
```

2. Using the `RPC_URL` environment variable:
```bash
RPC_URL=https://api.devnet.solana.com ./send-with-swap-cli send [OPTIONS]
```

## ⚡ Using with Just

The repository includes a Justfile for convenience with the following recipes:

```bash
# Initialize treasury
just init-treasury ~/.config/solana/id.json

# Send tokens
just send ~/.config/solana/id.json SOL USDC 0.1 9drLZh2B6oLNTJMKCcEF2VWfPRcT9HoAYTCcPpZ1h4xV

# Send with custom slippage (100 = 1%)
just send ~/.config/solana/id.json SOL USDC 0.1 9drLZh2B6oLNTJMKCcEF2VWfPRcT9HoAYTCcPpZ1h4xV "" 100

# Send on devnet
just send-devnet ~/.config/solana/id.json SOL USDC 0.1 9drLZh2B6oLNTJMKCcEF2VWfPRcT9HoAYTCcPpZ1h4xV
```

## 💱 Supported Tokens

Currently supported tokens:
- SOL
- USDC

More tokens can be added by updating the `get_token_mint_pubkey` function in the `helpers.rs` file.

## 🧑‍💻 Development

To add support for more tokens or customize the functionality, see the source code in the `src` directory:

- `main.rs`: Main CLI entry point with clap command definitions
- `helpers.rs`: Helper functions for treasury management and token conversions
- `retryable_rpc.rs`: Utilities for reliable RPC communication

## 📜 License

This project is open source and available under the MIT License.

---

Built with ❤️ by [Teapot in Wonderland](https://github.com/0x7EAB07) | [Twitter](https://twitter.com/0x7EAB07)
