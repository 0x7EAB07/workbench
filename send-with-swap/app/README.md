# ☕️ Send-with-Swap Apps

> Collection of clients for the Send-with-Swap Solana program, made with 💜 by [Teapot in Wonderland](https://github.com/0x7EAB07)

⚠️ **IMPORTANT: THESE TOOLS ARE CURRENTLY WORKS IN PROGRESS AND DO NOT FUNCTION YET. USE AT YOUR OWN RISK!** ⚠️

This directory contains client applications for interacting with the Send-with-Swap Solana program.

## 📝 Development Status

All applications in this directory are currently under active development and are **not yet functional**. They are being built to showcase the capabilities of the Send-with-Swap program but require additional work before they're ready for use.

## 📁 Contents

### CLI Client

A powerful command-line interface for the Send-with-Swap program, allowing you to:

- 💰 Initialize a treasury account for fee collection
- 💱 Swap tokens using Jupiter's swap aggregator while sending them to another wallet

The CLI client is located in the `cli-client/` directory.

### Building and Running

See the README in the cli-client directory for detailed instructions on building and using the CLI.

#### Quick Start

```bash
# Navigate to the CLI client directory
cd cli-client

# Build the CLI
cargo build --release

# Initialize a treasury account
just init-treasury ~/.config/solana/id.json

# Send tokens with swap
just send ~/.config/solana/id.json SOL USDC 0.1 <recipient-address>
```

## 🚀 Future Development

This app directory is intended to host different client implementations, including:

- 🌐 Web UI for the send-with-swap functionality
- 📱 Mobile app integrations
- 📚 SDK libraries for different languages

---

Built with ❤️ by [Teapot in Wonderland](https://github.com/0x7EAB07) | [Twitter](https://twitter.com/0x7EAB07)
