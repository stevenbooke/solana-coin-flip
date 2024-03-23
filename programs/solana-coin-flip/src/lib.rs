use anchor_lang::prelude::*;

declare_id!("3uQq4D1MdND4jz91Z6r5ZFETjZNBVwwjrTiMqgKzsvg7");

#[program]
pub mod solana_coin_flip {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
