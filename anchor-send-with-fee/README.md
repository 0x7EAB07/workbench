# Anchor Send With Fee 💸☕

> A curious Solana program for token transfers with fee deduction. Built by the Teapot in Wonderland.

## The Golden Key 🔑

This Anchor program demonstrates how to implement a savings mechanism that automatically saves a portion of your lamports during transfers on the Solana blockchain.

### Features

- Initialize a settings account to store fees
- Send lamports with automatic savings deduction
- Retrieve saved lamports at a later time
- Secure transaction processing

## Through the Looking Glass 🪞

### Prerequisites

- Rust and Cargo
- Solana CLI tools
- Anchor Framework
- Node.js and pnpm

### Quick Start

```bash
# Install dependencies
pnpm install

# Build the program
anchor build

# Test the program
anchor test

# Deploy to localnet
anchor deploy
```

## The White Rabbit's Pocketwatch ⏱️

### Program Flow

1. Initialize a settings account where fees will be saved
2. When sending lamports to another account, a portion is automatically saved
3. The saved lamports accumulate in the settings account
4. User can later retrieve all saved lamports when needed

This program acts as a savings account where you automatically save some lamports every time you send lamports to another account. With some modifications, this approach could be adapted for platform fees or other use cases.

## Tea Party Testing 🍵

Run the test suite to ensure everything works as expected:

```bash
anchor test
```

## Curiouser and Curiouser 🔍

### Project Structure

- `programs/`: Contains the Rust smart contract code
- `tests/`: Contains TypeScript tests
- `migrations/`: Deployment scripts

## Advice from a Caterpillar 🐛

### Security Considerations

- Ensure proper account validation
- Consider setting maximum and minimum savings amounts
- Implement access controls for fee retrieval
- Always test thoroughly before mainnet deployment

---

*"Begin at the beginning and go on till you come to the end; then stop." — The King of Hearts*
