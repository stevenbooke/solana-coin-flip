use anchor_lang::prelude::*;
use instructions::*;
use state::*;
use error::*;

pub mod instructions;
pub mod state;
pub mod error;

// pub use switchboard_solana::prelude::anchor_lang;
// pub use switchboard_solana::prelude::anchor_spl;

declare_id!("3uQq4D1MdND4jz91Z6r5ZFETjZNBVwwjrTiMqgKzsvg7");

#[program]
pub mod solana_coin_flip {
    use super::*;

    pub const PROGRAM_SEED: &[u8] = b"CUSTOMRANDOMNESS";
    pub const GUESS_COST: u64 = 100_000;

    pub fn create_user_account(ctx: Context<CreateUserAccount>) -> anchor_lang::Result<()> {
        create_user_account::create_user_account(ctx)
    }

    pub fn house_init(ctx: Context<HouseInit>, max_guess: u8) -> anchor_lang::Result<()> {
        house_init::house_init(ctx, max_guess)
    }

    pub fn house_set_function(ctx: Context<HouseSetFunction>) -> anchor_lang::Result<()> {
        house_set_function::house_set_function(ctx)
    }

    pub fn user_init(ctx: Context<UserInit>) -> anchor_lang::Result<()> {
        user_init::user_init(ctx)
    }

    pub fn user_guess(ctx: Context<UserGuess>, guess: u8, wager: u64) -> anchor_lang::Result<()> {
        user_guess::user_guess(ctx, guess, wager)
    }

    pub fn user_settle(ctx: Context<UserSettle>, result: u8) -> anchor_lang::Result<()> {
        user_settle::user_settle(ctx, result)
    }
}