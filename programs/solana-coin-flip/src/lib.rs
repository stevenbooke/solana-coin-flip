use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("2NqZ9e5ubnuvNj5VeBya5wdEEwKZSEZJRnZamDsdZdVB");

#[program]
pub mod solana_coin_flip {
    use super::*;

    pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {
        create_user_account::create_user_account(ctx)
    }

    pub fn request_randomness(ctx: Context<RequestRandomness>) -> anchor_lang::prelude::Result<()> {
        request_randomness::request_randomness(ctx)
    }

    pub fn consume_randomness(
        _ctx: Context<ConsumeRandomness>,
        result: Vec<u8>,
    ) -> anchor_lang::prelude::Result<()> {
        consume_randomness::consume_randomness(_ctx, result)
    }
}
