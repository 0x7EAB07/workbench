use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Numeric overflow")]
    NumericOverflow,
    #[msg("Not enough balance to withdraw")]
    NotEnoughBalanceToWithdraw,
}
