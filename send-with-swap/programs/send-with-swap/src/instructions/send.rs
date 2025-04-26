use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke},
};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{constants::TREASURY, jupiter_aggregator::program::Jupiter, treasury::Treasury};

#[derive(Accounts)]
pub struct Send<'info> {
    #[account(
        mut,
        seeds = [TREASURY.as_ref(), authority.key().as_ref()],
        bump = treasury.bump,
        has_one = authority, // treasury.authority == authority
        has_one = receiver, // treasury.receiver == receiver
    )]
    pub treasury: Account<'info, Treasury>,

    /// CHECK: Just to get the treasury
    pub authority: AccountInfo<'info>,

    /// CHECK: Just to get the receiver
    pub receiver: AccountInfo<'info>,
    #[account(
      mut,
      associated_token::token_program = output_mint_program,
      associated_token::mint = output_mint,
      associated_token::authority = receiver,
    )]
    pub output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
      mut,
      mint::token_program = output_mint_program,
      address = treasury.output_mint,
    )]
    pub output_mint: Box<InterfaceAccount<'info, Mint>>, // receiver_mint
    #[account(address = treasury.output_mint_program)]
    pub output_mint_program: Interface<'info, TokenInterface>,

    #[account(
      mut,
      associated_token::token_program = input_mint_program,
      associated_token::mint = input_mint,
      associated_token::authority = sender,
    )]
    pub input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
      mut,
      mint::token_program = input_mint_program,
    )]
    pub input_mint: Box<InterfaceAccount<'info, Mint>>,
    pub input_mint_program: Interface<'info, TokenInterface>,
    #[account(mut)]
    pub sender: Signer<'info>,

    pub system_program: Program<'info, System>,
    #[account(
      address = pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4")
    )]
    pub jupiter_program: Program<'info, Jupiter>,
}

pub fn processor(ctx: Context<Send>, swap_data: Vec<u8>) -> Result<()> {
    let swap_accounts: Vec<AccountMeta> = ctx
        .remaining_accounts
        .iter()
        .map(|acc| {
            let is_signer = acc.key == &ctx.accounts.sender.key();
            AccountMeta {
                pubkey: *acc.key,
                is_signer,
                is_writable: acc.is_writable,
            }
        })
        .collect();

    let swap_accounts_infos: Vec<AccountInfo> = ctx
        .remaining_accounts
        .iter()
        .map(|acc| AccountInfo { ..acc.clone() })
        .collect();

    invoke(
        &Instruction {
            program_id: ctx.accounts.jupiter_program.key(),
            accounts: swap_accounts,
            data: swap_data,
        },
        &swap_accounts_infos,
    )?;

    Ok(())
}
