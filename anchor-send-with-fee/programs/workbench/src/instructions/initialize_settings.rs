use crate::traits::account_space::AccountSpace;
use anchor_lang::prelude::*;

use crate::{Settings, SETTINGS};

#[derive(Accounts)]
pub struct InitializeSettings<'info> {
    #[account(
        init,
        payer = authority,
        space = Settings::account_space(),
        seeds = [SETTINGS.as_ref(), authority.key().as_ref()],
        bump,
    )]
    pub settings: Account<'info, Settings>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeSettingsArgs {
    pub fee: u64,
}

pub fn processor(ctx: Context<InitializeSettings>, args: InitializeSettingsArgs) -> Result<()> {
    let settings = &mut ctx.accounts.settings;
    settings.set_inner(Settings::new(
        ctx.bumps.settings,
        ctx.accounts.authority.key(),
        args.fee,
    ));
    Ok(())
}
