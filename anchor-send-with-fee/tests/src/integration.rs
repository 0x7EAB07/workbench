use std::error::Error;

use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signature},
        signer::Signer,
    },
    Client, ClientError, Cluster, Program,
};
use workbench::{
    accounts,
    instruction::{InitializeSettings, TransferWithFee, Withdraw},
    settings_seeds_without_bump, InitializeSettingsArgs, TransferWithFeeArgs,
    ID as WorkbenchProgramID, SETTINGS,
};

fn get_settings_pda(payer: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(settings_seeds_without_bump!(payer), program_id)
}

fn initialize_settings(
    program: &Program<&Keypair>,
    payer: &Keypair,
    settings: Pubkey,
) -> Result<Signature, ClientError> {
    program
        .request()
        .accounts(accounts::InitializeSettings {
            authority: payer.pubkey(),
            settings,
            system_program: system_program::ID,
        })
        .args(InitializeSettings {
            args: InitializeSettingsArgs { fee: 5_000_000 },
        })
        .send()
}

fn transfer_with_fee(
    program: &Program<&Keypair>,
    payer: &Keypair,
    recipient: Pubkey,
    settings: Pubkey,
    amount: u64,
) -> Result<Signature, ClientError> {
    program
        .request()
        .accounts(accounts::TransferWithFee {
            authority: payer.pubkey(),
            from: payer.pubkey(),
            to: recipient,
            settings,
            system_program: system_program::ID,
        })
        .signer(payer) // Add signer here since we're transferring from this account
        .args(TransferWithFee {
            args: TransferWithFeeArgs { amount },
        })
        .send()
}

fn withdraw(
    program: &Program<&Keypair>,
    payer: &Keypair,
    settings: Pubkey,
) -> Result<Signature, ClientError> {
    program
        .request()
        .accounts(accounts::Withdraw {
            settings,
            authority: payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(Withdraw {})
        .send()
}

#[test]
fn test_integration() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
    let recipient = Keypair::new();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program = client.program(WorkbenchProgramID).unwrap();

    let (settings, _) = get_settings_pda(&payer.pubkey(), &program.id());

    // Step 1: Initialize settings
    let tx = initialize_settings(&program, &payer, settings).unwrap();
    println!(
        "Initialize settings transaction signature: {:?}",
        tx.to_string()
    );

    // Step 2: Transfer with fee
    let tx = transfer_with_fee(&program, &payer, recipient.pubkey(), settings, 10_000_000).unwrap();
    println!(
        "Transfer with fee transaction signature: {:?}",
        tx.to_string()
    );

    // Step 3: Withdraw the collected fees
    let tx = withdraw(&program, &payer, settings).unwrap();
    println!("Withdraw transaction signature: {:?}", tx.to_string());
}
