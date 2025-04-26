mod helpers;
mod retryable_rpc;

use anyhow::{anyhow, Result};
use borsh::BorshSerialize;
use clap::{Parser, Subcommand};
use jupiter_swap_api_client::{
    QuoteGetRequest, QuoteResponse, SwapApiClient, SwapLegAndAccountMetas, SwapRequest, SwapResponse,
};
use retryable_rpc::RetryableRpcClient;
use sha2::{Digest, Sha256};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::sync_native;
use std::{env, str::FromStr};

// Define constants
const SEND_WITH_SWAP_PROGRAM_ID: &str = "HFtu7KcUGWw7TDHKdsMjFK2SZfzWav3AtQqhSskPFMdT";
const JUPITER_PROGRAM_ID: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";

/// Send-with-Swap CLI - A tool for swapping and sending tokens on Solana
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional custom RPC URL (can also be set via RPC_URL env var)
    #[arg(short, long)]
    rpc_url: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new treasury account for fee collection
    InitTreasury {
        /// Path to the payer's keypair file
        #[arg(short, long, value_name = "KEYPAIR_PATH")]
        keypair: String,
    },

    /// Send tokens to another wallet after swapping them
    Send {
        /// Path to the payer's keypair file
        #[arg(short, long, value_name = "KEYPAIR_PATH")]
        keypair: String,

        /// Input token symbol (e.g., SOL, USDC)
        #[arg(short, long)]
        input_token: String,

        /// Output token symbol (e.g., SOL, USDC)
        #[arg(short, long)]
        output_token: String,

        /// Amount of input token to swap
        #[arg(short, long)]
        amount: f64,

        /// Recipient's wallet address (base58 encoded)
        #[arg(short, long)]
        recipient: String,

        /// Optional treasury pubkey (if not provided, will be derived from program ID)
        #[arg(short, long)]
        treasury: Option<String>,

        /// Slippage in basis points (default: 50 = 0.5%)
        #[arg(short, long, default_value_t = 50)]
        slippage_bps: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Display warning about WIP status
    print_wip_warning();

    // Parse command line arguments using clap
    let cli = Cli::parse();

    // Get RPC URL from command line or environment variable
    let rpc_url = cli.rpc_url
        .or_else(|| env::var("RPC_URL").ok())
        .unwrap_or_else(|| "https://api.mainnet-beta.solana.com".to_string());

    // Additional warning specific to the command
    match &cli.command {
        Commands::InitTreasury { .. } => {
            println!("\n⚠️  The treasury initialization functionality is not yet implemented.");
            println!("    This command will simulate the process but won't perform any on-chain operations.\n");
        },
        Commands::Send { .. } => {
            println!("\n⚠️  The send-with-swap functionality is not yet implemented.");
            println!("    This command will simulate the process but won't perform any on-chain operations.\n");
        },
    }

    // Dispatch to the appropriate command
    match cli.command {
        Commands::InitTreasury { keypair } => {
            init_treasury_command(&keypair, &rpc_url).await
        },
        Commands::Send {
            keypair,
            input_token,
            output_token,
            amount,
            recipient,
            treasury,
            slippage_bps
        } => {
            let recipient_pubkey = Pubkey::from_str(&recipient)
                .map_err(|_| anyhow!("Invalid recipient pubkey"))?;

            // Treasury is optional - if not provided, we try to find it
            let treasury_pubkey = if let Some(treasury_str) = treasury {
                Pubkey::from_str(&treasury_str)?
            } else {
                // Auto-detect treasury
                let program_id = Pubkey::from_str(SEND_WITH_SWAP_PROGRAM_ID)?;
                let treasury_seeds = &[b"treasury"];
                let (treasury, _) = Pubkey::find_program_address(treasury_seeds, &program_id);
                treasury
            };

            send_command(
                &keypair,
                &input_token,
                &output_token,
                amount,
                recipient_pubkey,
                treasury_pubkey,
                slippage_bps,
                &rpc_url
            ).await
        },
    }
}

fn print_wip_warning() {
    println!("\n");
    println!("⚠️ ⚠️ ⚠️  WARNING: WORK IN PROGRESS  ⚠️ ⚠️ ⚠️");
    println!("────────────────────────────────────────────");
    println!("This tool is currently under active development and is NOT FUNCTIONAL yet.");
    println!("It's provided for demonstration and development purposes only.");
    println!("No actual transactions will be executed on the blockchain.");
    println!("────────────────────────────────────────────\n");
}

async fn init_treasury_command(keypair_path: &str, rpc_url: &str) -> Result<()> {
    // Load keypair
    let payer = read_keypair_file(keypair_path).map_err(|_| anyhow!("Failed to read keypair file"))?;

    // Connect to Solana
    let commitment = CommitmentConfig::confirmed();
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), commitment);

    // Get program ID
    let program_id = Pubkey::from_str(SEND_WITH_SWAP_PROGRAM_ID)?;

    // Calculate the treasury PDA (for simulation purposes)
    let treasury_seeds = &[b"treasury"];
    let (treasury_pubkey, _) = Pubkey::find_program_address(treasury_seeds, &program_id);

    println!("[SIMULATION] Treasury would be initialized with address: {}", treasury_pubkey);
    println!("[SIMULATION] This would require a transaction from: {}", payer.pubkey());
    println!("\nNOTE: No actual transaction was sent. This is currently just a simulation.");

    Ok(())
}

async fn send_command(
    keypair_path: &str,
    input_token: &str,
    output_token: &str,
    amount: f64,
    recipient: Pubkey,
    treasury_pubkey: Pubkey,
    slippage_bps: u64,
    rpc_url: &str,
) -> Result<()> {
    // Load the payer's keypair
    let payer = read_keypair_file(keypair_path).map_err(|_| anyhow!("Failed to read keypair file"))?;

    // Connect to Solana with retry capability
    let commitment = CommitmentConfig::confirmed();
    let retryable_client = RetryableRpcClient::new(rpc_url, commitment, 3, 1000);
    let client = retryable_client.get_client();

    // Initialize the Jupiter Swap API client
    let swap_api_client = SwapApiClient::new();

    // Convert input amount to proper decimals based on token
    let input_amount = match input_token.to_uppercase().as_str() {
        "SOL" => (amount * 1_000_000_000.0) as u64, // 9 decimals
        "USDC" => (amount * 1_000_000.0) as u64,    // 6 decimals
        _ => return Err(anyhow!("Unsupported input token")),
    };

    println!("[SIMULATION] Would swap {} {} to {} and send to {}",
        amount,
        input_token,
        output_token,
        recipient
    );

    println!("[SIMULATION] Would use treasury: {}", treasury_pubkey);
    println!("[SIMULATION] Slippage tolerance would be: {}bps ({}%)",
        slippage_bps,
        slippage_bps as f64 / 100.0
    );

    println!("\nNOTE: No actual transaction was sent. This is currently just a simulation.");

    Ok(())
}

async fn process_transaction(
    client: &RpcClient,
    payer: &Keypair,
    swap_response: &SwapResponse,
    send_with_swap_program_id: &Pubkey,
    recipient: Pubkey,
    treasury_pubkey: Pubkey,
) -> Result<String> {
    // This is kept for future implementation
    println!("[SIMULATION] Would process the transaction here");
    Ok("simulated_transaction_signature".to_string())
}

fn serialize_send_instruction(swap_data: Vec<u8>) -> Result<Vec<u8>> {
    // Based on the Anchor format for the send instruction
    // This will need to match your program's instruction format
    let mut data = vec![2]; // Assuming instruction index for "send" is 2 (0-indexed)
    data.extend_from_slice(&(swap_data.len() as u32).to_le_bytes());
    data.extend_from_slice(&swap_data);
    Ok(data)
}

fn format_token_amount(amount_str: &str, token: &str) -> String {
    let amount: f64 = amount_str.parse().unwrap_or(0.0);
    match token.to_uppercase().as_str() {
        "SOL" => (amount / 1_000_000_000.0).to_string(),
        "USDC" => (amount / 1_000_000.0).to_string(),
        _ => amount.to_string(),
    }
}
