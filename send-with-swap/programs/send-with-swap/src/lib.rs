pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod traits;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_program!(jupiter_aggregator);
declare_id!("HFtu7KcUGWw7TDHKdsMjFK2SZfzWav3AtQqhSskPFMdT");

#[program]
pub mod send_with_swap {
    use super::*;

    pub fn init_treasury(ctx: Context<InitTreasury>) -> Result<()> {
        init_treasury::processor(ctx)
    }

    pub fn update_treasury(ctx: Context<UpdateTreasury>) -> Result<()> {
        update_treasury::processor(ctx)
    }

    pub fn send(ctx: Context<Send>, swap_data: Vec<u8>) -> Result<()> {
        send::processor(ctx, swap_data)
    }
}
