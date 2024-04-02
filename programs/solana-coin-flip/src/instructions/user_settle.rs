use anchor_lang::prelude::*;

use switchboard_solana::prelude::*;
use crate::state::{HouseState, UserState, RoundStatus};
use crate::PROGRAM_SEED;
use crate::RandomnessRequestError;

#[derive(Accounts)]
pub struct UserSettle<'info> {
    // CLIENT ACCOUNTS
    #[account(
      seeds = [PROGRAM_SEED],
      bump = house.load()?.bump,
      has_one = function,
    )]
    pub house: AccountLoader<'info, HouseState>,

    #[account(
        mut,
        seeds = [PROGRAM_SEED, user.load()?.authority.as_ref()],
        bump = user.load()?.bump,
        constraint = user.load()?.token_wallet == user_token_wallet.key(),
    )]
    pub user: AccountLoader<'info, UserState>,

    // SWITCHBOARD ACCOUNTS
    pub function: AccountLoader<'info, FunctionAccountData>,
    #[account(
        constraint = request.validate_signer(
            &function,
            &enclave_signer.to_account_info(),
        )?
    )]
    pub request: Box<Account<'info, FunctionRequestAccountData>>,
    pub enclave_signer: Signer<'info>,

    // TOKEN ACCOUNTS
    pub token_program: Program<'info, Token>,
    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub house_token_wallet: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_token_wallet: Box<Account<'info, TokenAccount>>,
}

pub fn user_settle(ctx: Context<UserSettle>, result: u8) -> anchor_lang::Result<()> {
    // verify we havent responded already
    if ctx.accounts.user.load()?.current_round.status != RoundStatus::Pending {
        return Err(error!(RandomnessRequestError::RoundInactive));
    }

    if ctx.accounts.request.active_request.status != RequestStatus::RequestSuccess {
        return Err(error!(
            RandomnessRequestError::SwitchboardRequestNotSuccessful
        ));
    }

    let mut user = ctx.accounts.user.load_mut()?;
    user.current_round.result = result;
    user.current_round.status = RoundStatus::Settled;

    // TODO: payout

    Ok(())
}