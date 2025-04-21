use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{error::ErrorCode, settings_seeds, traits::AccountSpace, Settings, SETTINGS};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
      mut,
      seeds = [SETTINGS.as_ref(), authority.key().as_ref()],
      bump = settings.bump,
    )]
    pub settings: Account<'info, Settings>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn processor(ctx: Context<Withdraw>) -> Result<()> {
    let rent = Rent::get()?;
    let settings = &mut ctx.accounts.settings;

    let exceeding_lamports = settings
        .get_lamports()
        .saturating_sub(rent.minimum_balance(Settings::account_space()));

    require_gt!(exceeding_lamports, 0, ErrorCode::NotEnoughBalanceToWithdraw);

    let settings_seeds = settings_seeds!(settings);
    let settings_signer = &[&settings_seeds[..]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: settings.to_account_info(),
                to: ctx.accounts.authority.to_account_info(),
            },
            settings_signer,
        ),
        exceeding_lamports,
    )?;

    Ok(())
}
