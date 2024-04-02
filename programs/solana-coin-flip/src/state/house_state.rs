use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
pub struct HouseState {
    pub bump: u8,
    pub max_guess: u8,
    pub authority: Pubkey,
    pub function: Pubkey,
    pub token_wallet: Pubkey,
}