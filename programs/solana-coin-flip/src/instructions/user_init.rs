use anchor_lang::prelude::*;

use switchboard_solana::prelude::*;
use crate::state::{HouseState, UserState};
use crate::PROGRAM_SEED;

#[derive(Accounts)]
pub struct UserInit<'info> {
    // PROGRAM ACCOUNTS
    #[account(
      seeds = [PROGRAM_SEED],
      bump = house.load()?.bump,
      constraint = house.load()?.token_wallet == house_token_wallet.key(),
    )]
    pub house: AccountLoader<'info, HouseState>,

    #[account(
        init,
        space = 8 + std::mem::size_of::<UserState>(),
        payer = payer,
        seeds = [PROGRAM_SEED, payer.key().as_ref()],
        bump
    )]
    pub user: AccountLoader<'info, UserState>,

    // TOKEN ACCOUNTS
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub mint: Account<'info, Mint>,
    #[account(
      associated_token::mint = mint,
      associated_token::authority = house,
    )]
    pub house_token_wallet: Box<Account<'info, TokenAccount>>,
    #[account(
      init,
      payer = payer,
      associated_token::mint = mint,
      associated_token::authority = user,
    )]
    pub user_token_wallet: Box<Account<'info, TokenAccount>>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

pub fn user_init(ctx: Context<UserInit>) -> anchor_lang::Result<()> {
    let mut user = ctx.accounts.user.load_init()?;
    // user.bump = *ctx.bumps.get("user").unwrap();
    user.bump = ctx.bumps.user;
    user.authority = ctx.accounts.payer.key();
    user.token_wallet = ctx.accounts.user_token_wallet.key();

    Ok(())
}