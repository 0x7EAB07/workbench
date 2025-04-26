use anyhow::{anyhow, Result};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;

// Treasury account creation helper
pub async fn create_treasury(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &Pubkey,
) -> Result<Pubkey> {
    // Find treasury PDA
    let treasury_seeds = &[b"treasury"];
    let (treasury_pubkey, _) = Pubkey::find_program_address(treasury_seeds, program_id);

    // Check if treasury already exists
    if let Ok(_) = client.get_account(&treasury_pubkey).await {
        println!("Treasury account already exists: {}", treasury_pubkey);
        return Ok(treasury_pubkey);
    }

    // Create init_treasury instruction
    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(treasury_pubkey, false),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
    ];

    // Initialize treasury instruction (index 0)
    let data = vec![0];
    let instruction = Instruction {
        program_id: *program_id,
        accounts,
        data,
    };

    // Execute the transaction
    let recent_blockhash = client.get_latest_blockhash().await?;
    let message = solana_sdk::message::Message::new(&[instruction], Some(&payer.pubkey()));
    let transaction = Transaction::new(&[payer], message, recent_blockhash);

    let signature = client.send_and_confirm_transaction(&transaction).await?;
    println!("Created treasury account in transaction: {}", signature);

    Ok(treasury_pubkey)
}

// Get the Jupiter account data structure
pub fn get_jupiter_accounts(jupiter_ix_data: &[u8], accounts: &[AccountMeta], program_id: &Pubkey) -> Vec<AccountMeta> {
    // This is a utility function to help structure accounts for the Jupiter CPI
    let mut result = vec![];

    // Add accounts based on the program's account requirement
    // In a real-world scenario, you'd need to match exactly what your program expects

    for account in accounts {
        result.push(account.clone());
    }

    result
}

// Convert token names to mint addresses
pub fn get_token_mint_pubkey(token: &str) -> Result<Pubkey> {
    match token.to_uppercase().as_str() {
        "SOL" => Ok(Pubkey::from_str("So11111111111111111111111111111111111111112")?),
        "USDC" => Ok(Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")?),
        // Add more tokens as needed
        _ => Err(anyhow!("Unsupported token: {}", token)),
    }
}

// Check if a transaction was successful
pub async fn check_transaction_status(client: &RpcClient, signature: &str) -> Result<bool> {
    let sig = solana_sdk::signature::Signature::from_str(signature)?;
    let status = client.get_signature_statuses(&[sig]).await?;

    match status.value[0] {
        Some(ref stat) if stat.err.is_none() => Ok(true),
        Some(ref stat) => {
            println!("Transaction failed with error: {:?}", stat.err);
            Ok(false)
        },
        None => {
            println!("Transaction status not found");
            Ok(false)
        },
    }
}
