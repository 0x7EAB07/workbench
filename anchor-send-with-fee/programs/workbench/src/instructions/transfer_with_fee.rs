use anchor_lang::{prelude::*, system_program::transfer};

use crate::{Settings, SETTINGS};

#[derive(Accounts)]
pub struct TransferWithFee<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: SystemAccount<'info>,

    #[account(
      seeds = [SETTINGS.as_ref(), from.key().as_ref()],
      bump = settings.bump,
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
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.settings.to_account_info(),
            },
        ),
        ctx.accounts.settings.fee,
    )?;

    Ok(())
}
