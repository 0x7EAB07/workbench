use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
    },
    Client, Cluster, Program,
};
use workbench::{
    accounts, instruction::{InitializeSettings, TransferWithFee, Withdraw},
    settings_seeds_without_bump, InitializeSettingsArgs, TransferWithFeeArgs,
    ID as WorkbenchProgramID, SETTINGS,
};

fn initialize_settings(
    program: &Program<&Keypair>,
    payer: &Keypair,
) -> Result<String, Box<dyn std::error::Error>> {
    let (settings, _) =
        Pubkey::find_program_address(settings_seeds_without_bump!(payer.pubkey()), &program.id());

    let tx = program
        .request()
        .accounts(accounts::InitializeSettings {
            authority: payer.pubkey(),
            settings: settings,
            system_program: system_program::ID,
        })
        .args(InitializeSettings {
            args: InitializeSettingsArgs { fee: 5_000_000 },
        })
        .send()?;

    Ok(tx.to_string())
}

fn transfer_with_fee(
    program: &Program<&Keypair>,
    payer: &Keypair,
    recipient: &Pubkey,
    amount: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let (settings, _) =
        Pubkey::find_program_address(settings_seeds_without_bump!(payer.pubkey()), &program.id());

    let tx = program
        .request()
        .accounts(accounts::TransferWithFee {
            from: payer.pubkey(),
            to: *recipient,
            settings,
            system_program: system_program::ID,
        })
        .args(TransferWithFee {
            args: TransferWithFeeArgs { amount },
        })
        .send()?;

    Ok(tx.to_string())
}

fn withdraw(
    program: &Program<&Keypair>,
    payer: &Keypair,
) -> Result<String, Box<dyn std::error::Error>> {
    let (settings, _) =
        Pubkey::find_program_address(settings_seeds_without_bump!(payer.pubkey()), &program.id());

    let tx = program
        .request()
        .accounts(accounts::Withdraw {
            settings,
            authority: payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(Withdraw {})
        .send()?;

    Ok(tx.to_string())
}

#[test]
fn test_integration() {
    let anchor_wallet: String = std::env::var("ANCHOR_WALLET").unwrap();
    let payer: Keypair = read_keypair_file(&anchor_wallet).unwrap();

    let client: Client<&Keypair> =
        Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program: Program<&Keypair> = client.program(WorkbenchProgramID).unwrap();

    // Step 1: Initialize settings
    let tx = initialize_settings(&program, &payer).unwrap();
    println!("Initialize settings transaction signature: {:?}", tx);

    // Step 2: Create a recipient for transfer
    let recipient = Keypair::new();

    // Step 3: Transfer with fee
    let tx = transfer_with_fee(&program, &payer, &recipient.pubkey(), 10_000_000).unwrap();
    println!("Transfer with fee transaction signature: {:?}", tx);

    // Step 4: Withdraw the collected fees
    let tx = withdraw(&program, &payer).unwrap();
    println!("Withdraw transaction signature: {:?}", tx);
}
