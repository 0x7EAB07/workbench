use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{constants::TREASURY, treasury::Treasury};

#[derive(Accounts)]
pub struct UpdateTreasury<'info> {
    #[account(
      mut,
      seeds = [TREASURY.as_ref(), authority.key().as_ref()],
      bump = treasury.bump,
      has_one = authority,
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: Just to set the recipient of funds
    pub recipient: AccountInfo<'info>,
    #[account(
      mint::token_program = output_mint_program,
    )]
    pub output_mint: InterfaceAccount<'info, Mint>,
    /// CHECK: Just to set the recipient mint token program
    pub output_mint_program: Interface<'info, TokenInterface>,
}

pub fn processor(ctx: Context<UpdateTreasury>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    treasury.update_recipient(
        ctx.accounts.recipient.key(),
        ctx.accounts.output_mint.key(),
        ctx.accounts.output_mint_program.key(),
    );
    Ok(())
}
