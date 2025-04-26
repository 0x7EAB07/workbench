# Send With Swap 🪄🔄💰

> A magical Solana program that enables token swaps during transfers using Jupiter aggregator. Built by Teapot in Wonderland.

## The Looking Glass 🔍

This Anchor program demonstrates how to implement seamless token swaps during transfers on the Solana blockchain, utilizing Jupiter as the swap aggregator. Send tokens in one currency and have them arrive in another - like magic!

### Features

- Initialize a treasury to store swap settings
- Send tokens with automatic swapping to a specified output token
- Configurable receiver and output token
- Integrated with Jupiter aggregator for optimal swap routes
- Secure transaction processing

## The White Rabbit's Guide 🐇

### Prerequisites

- Rust and Cargo
- Solana CLI tools
- Anchor Framework
- Node.js and pnpm

## The Cheshire Cat's Explanation 😺

### Program Flow

1. Initialize a treasury account that defines swap settings
2. When sending tokens, the program:
   - Takes input tokens from the sender
   - Swaps them through Jupiter to the desired output token
   - Delivers the swapped tokens to the receiver
3. The treasury can be updated to change the receiver or output token

This program acts as a magical transport system where tokens transform from one type to another during their journey. It can be used for recurring payments in a specific token, regardless of what token the sender holds.

## The Queen's Architecture 👑

### Project Structure

- `programs/send-with-swap/src/`: Contains the Rust smart contract code
  - `instructions/`: Contains the program instructions
  - `state/`: Contains program state definitions
  - `constants.rs`: Program constants
  - `error.rs`: Custom error definitions
- `tests/`: Contains program tests
- `app/`: Client implementation *Super important*
- Test extensively before mainnet deployment

---

*"One token to swap them all, one program to find them, one Jupiter to bring them all, and in the blockchain bind them." — Teapot in Wonderland*
