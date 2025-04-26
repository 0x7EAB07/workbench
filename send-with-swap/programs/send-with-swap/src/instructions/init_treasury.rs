use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{constants::TREASURY, traits::AccountSpace, treasury::Treasury};

#[derive(Accounts)]
pub struct InitTreasury<'info> {
    #[account(
      init,
      payer = payer,
      space = Treasury::account_space(),
      seeds = [TREASURY.as_ref(), authority.key().as_ref()],
      bump
    )]
    pub treasury: Account<'info, Treasury>,

    /// CHECK: Account defining the authority of the treasury
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Just to set the recipient of funds
    pub recipient: AccountInfo<'info>,
    #[account(
      mint::token_program = output_mint_program,
    )]
    pub recipient_mint: InterfaceAccount<'info, Mint>,
    /// CHECK: Just to set the recipient mint token program
    pub output_mint_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

pub fn processor(ctx: Context<InitTreasury>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    treasury.set_inner(Treasury::new(
        ctx.bumps.treasury,
        ctx.accounts.authority.key(),
        ctx.accounts.recipient.key(),
        ctx.accounts.recipient_mint.key(),
        ctx.accounts.output_mint_program.key(),
    ));
    Ok(())
}
