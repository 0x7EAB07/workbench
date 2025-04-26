use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub bump: u8,
    pub authority: Pubkey,
    pub receiver: Pubkey,
    pub output_mint: Pubkey,
    pub output_mint_program: Pubkey,
}

impl Treasury {
    pub fn new(
        bump: u8,
        authority: Pubkey,
        receiver: Pubkey,
        output_mint: Pubkey,
        output_mint_program: Pubkey,
    ) -> Self {
        Self {
            bump,
            authority,
            receiver,
            output_mint,
            output_mint_program,
        }
    }

    pub fn update_recipient(
        &mut self,
        receiver: Pubkey,
        output_mint: Pubkey,
        output_mint_program: Pubkey,
    ) {
        self.receiver = receiver;
        self.output_mint = output_mint;
        self.output_mint_program = output_mint_program;
    }
}

#[macro_export]
macro_rules! treasury_seeds_without_bump {
    ($authority:expr) => {
        &[TREASURY, $authority.as_ref()]
    };
}

#[macro_export]
macro_rules! treasury_seeds {
    ($settings:expr) => {
        &[TREASURY, $treasury.authority.as_ref(), &[$treasury.bump]]
    };
}
