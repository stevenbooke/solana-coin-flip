// use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;

use crate::state::HouseState;
use crate::PROGRAM_SEED;

#[derive(Accounts)]
pub struct HouseInit<'info> {
    // PROGRAM ACCOUNTS
    #[account(
        init,
        space = 8 + std::mem::size_of::<HouseState>(),
        payer = payer,
        seeds = [PROGRAM_SEED],
        bump
    )]
    pub house: AccountLoader<'info, HouseState>,

    /// CHECK: make sure this matches function
    pub authority: AccountInfo<'info>,

    // SWITCHBOARD ACCOUNTS
    #[account(
      has_one = escrow_wallet,
      // TODO: verify authority
    )]
    pub function: AccountLoader<'info, FunctionAccountData>,
    #[account(has_one = token_wallet, has_one = mint)]
    pub escrow_wallet: Box<Account<'info, SwitchboardWallet>>,
    #[account(constraint = token_wallet.mint == mint.key())]
    pub token_wallet: Box<Account<'info, TokenAccount>>,

    // TOKEN ACCOUNTS
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub mint: Account<'info, Mint>,
    #[account(
      init,
      payer = payer,
      associated_token::mint = mint,
      associated_token::authority = house,
  )]
    pub house_token_wallet: Box<Account<'info, TokenAccount>>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

pub fn house_init(ctx: Context<HouseInit>, max_guess: u8) -> anchor_lang::Result<()> {
    let mut house = ctx.accounts.house.load_init()?;
    // house.bump = *ctx.bumps.get("house").unwrap();
    house.bump = ctx.bumps.house;
    house.authority = ctx.accounts.authority.key();
    house.function = ctx.accounts.function.key();
    house.token_wallet = ctx.accounts.house_token_wallet.key();
    house.max_guess = max_guess;

    Ok(())
}