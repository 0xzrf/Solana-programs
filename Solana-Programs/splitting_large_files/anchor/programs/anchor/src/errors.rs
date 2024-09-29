use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode{
    #[msg("Something unexpected happened")]
    SomethingUnexpected
}