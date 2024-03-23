use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("3uQq4D1MdND4jz91Z6r5ZFETjZNBVwwjrTiMqgKzsvg7");

#[program]
pub mod solana_coin_flip {
    use super::*;

    pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {
        create_user_account::create_user_account(ctx)
    }
}
