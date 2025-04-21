use anchor_lang::prelude::*;

// Define SETTINGS constant manually
pub const SETTINGS: &[u8] = b"settings";

#[account]
#[derive(InitSpace)]
pub struct Settings {
    pub bump: u8,
    pub authority: Pubkey,
    pub fee: u64,
}

impl Settings {
    pub fn new(bump: u8, authority: Pubkey, fee: u64) -> Self {
        Self {
            bump,
            authority,
            fee,
        }
    }
}

#[macro_export]
macro_rules! settings_seeds_without_bump {
    ($authority:expr) => {
        &[SETTINGS, $authority.as_ref()]
    };
}

#[macro_export]
macro_rules! settings_seeds {
    ($settings:expr) => {
        &[SETTINGS, $settings.authority.as_ref(), &[$settings.bump]]
    };
}
