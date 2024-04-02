// use anchor_lang::prelude::*;
pub use switchboard_solana::prelude::*;

use crate::state::HouseState;
use crate::PROGRAM_SEED;

#[derive(Accounts)]
pub struct HouseSetFunction<'info> {
    // PROGRAM ACCOUNTS
    #[account(
        mut,
        seeds = [PROGRAM_SEED],
        bump = house.load()?.bump,
        has_one = authority,
    )]
    pub house: AccountLoader<'info, HouseState>,

    /// CHECK:
    pub authority: Signer<'info>,

    // SWITCHBOARD ACCOUNTS
    pub function: AccountLoader<'info, FunctionAccountData>,
}

pub fn house_set_function(ctx: Context<HouseSetFunction>) -> anchor_lang::Result<()> {
    let mut house = ctx.accounts.house.load_mut()?;
    house.function = ctx.accounts.function.key();

    Ok(())
}