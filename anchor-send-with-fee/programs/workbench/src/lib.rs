#![allow(ambiguous_glob_reexports)]
#![allow(unexpected_cfgs)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod traits;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("DJjESvYqYLBWGK5Zc2XBxPVWFEyDe6fN65NNyaf6WXRn");

#[program]
pub mod workbench {
    use super::*;

    pub fn initialize_settings(
        ctx: Context<InitializeSettings>,
        args: InitializeSettingsArgs,
    ) -> Result<()> {
        initialize_settings::processor(ctx, args)
    }

    pub fn transfer_with_fee(
        ctx: Context<TransferWithFee>,
        args: TransferWithFeeArgs,
    ) -> Result<()> {
        transfer_with_fee::processor(ctx, args)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw::processor(ctx)
    }
}
