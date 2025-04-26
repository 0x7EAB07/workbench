use anchor_lang::{prelude::*, system_program::transfer};

use crate::{Settings, SETTINGS};

#[derive(Accounts)]
pub struct TransferWithFee<'info> {
    /// CHECK: This is the authority account that is used to transfer the fee
    #[account(mut)]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub from: Signer<'info>,

    /// CHECK: This is the recipient account that is used to transfer the main amount
    #[account(mut)]
    pub to: AccountInfo<'info>,

    #[account(
      mut,
      seeds = [SETTINGS.as_ref(), authority.key().as_ref()],
      bump = settings.bump,
      has_one = authority,
    )]
    pub settings: Account<'info, Settings>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TransferWithFeeArgs {
    pub amount: u64,
}

pub fn processor(ctx: Context<TransferWithFee>, args: TransferWithFeeArgs) -> Result<()> {
    // Transfer main amount to recipient
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
            },
        ),
        args.amount,
    )?;

    // Transfer fee to settings account
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.authority.to_account_info(),
                to: ctx.accounts.settings.to_account_info(),
            },
        ),
        ctx.accounts.settings.fee,
    )?;

    Ok(())
}
