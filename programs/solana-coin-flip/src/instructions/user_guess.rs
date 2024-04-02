use anchor_lang::prelude::*;

use switchboard_solana::prelude::*;
use crate::state::{HouseState, UserState, UserRound, RoundStatus};
use crate::{GUESS_COST, PROGRAM_SEED};
use switchboard_solana::wrap_native;
use crate::RandomnessRequestError;

#[derive(Accounts)]
pub struct UserGuess<'info> {
    // PROGRAM ACCOUNTS
    #[account(
      seeds = [PROGRAM_SEED],
      bump = house.load()?.bump,
      has_one = function,
      constraint = house.load()?.token_wallet == house_token_wallet.key(),
    )]
    pub house: AccountLoader<'info, HouseState>,

    #[account(
        mut,
        seeds = [PROGRAM_SEED, payer.key().as_ref()], // user should be paying for this each time
        bump = user.load()?.bump,
        constraint = user.load()?.authority == payer.key() && user.load()?.token_wallet == user_token_wallet.key(),
    )]
    pub user: AccountLoader<'info, UserState>,

    // SWITCHBOARD ACCOUNTS
    /// CHECK:
    #[account(executable, address = SWITCHBOARD_ATTESTATION_PROGRAM_ID)]
    pub switchboard: AccountInfo<'info>,
    #[account(
      seeds = [STATE_SEED],
      seeds::program = switchboard.key(),
      bump = state.load()?.bump,
    )]
    pub state: AccountLoader<'info, AttestationProgramState>,
    pub attestation_queue: AccountLoader<'info, AttestationQueueAccountData>,
    #[account(
      mut,
      has_one = attestation_queue,
    )]
    pub function: AccountLoader<'info, FunctionAccountData>,
    /// CHECK:
    #[account(
      mut,
      signer,
      owner = system_program.key(),
      constraint = request.data_len() == 0 && request.lamports() == 0
    )]
    pub request: AccountInfo<'info>,
    /// CHECK:
    #[account(
      mut,
      owner = system_program.key(),
      constraint = request.data_len() == 0 && request.lamports() == 0
    )]
    pub request_escrow: AccountInfo<'info>,

    // TOKEN ACCOUNTS
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub mint: Account<'info, Mint>,
    pub house_token_wallet: Box<Account<'info, TokenAccount>>,
    #[account(mut)] // we might wrap funds to this wallet
    pub user_token_wallet: Box<Account<'info, TokenAccount>>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

pub fn user_guess(ctx: Context<UserGuess>, guess: u8, wager: u64) -> anchor_lang::Result<()> {
    if ctx.accounts.house_token_wallet.amount < GUESS_COST {
        return Err(error!(RandomnessRequestError::HouseInsufficientFunds));
    }

    if ctx.accounts.user_token_wallet.amount < GUESS_COST {
        wrap_native(
            &ctx.accounts.system_program,
            &ctx.accounts.token_program,
            &ctx.accounts.user_token_wallet,
            &ctx.accounts.payer,
            &[&[
                PROGRAM_SEED,
                ctx.accounts.user.load()?.authority.key().as_ref(),
                &[ctx.accounts.user.load()?.bump],
            ]],
            GUESS_COST
                .checked_sub(ctx.accounts.user_token_wallet.amount)
                .unwrap(),
        )?;
    }

    ctx.accounts.user_token_wallet.reload()?;

    assert!(
        ctx.accounts.user_token_wallet.amount >= GUESS_COST,
        "User escrow is missing funds"
    );

    let request_params = format!(
        "PID={},MAX_GUESS={},USER={}",
        crate::id(),
        ctx.accounts.house.load()?.max_guess,
        ctx.accounts.user.key()
    );

    let request_init_ctx = FunctionRequestInitAndTrigger {
        request: ctx.accounts.request.clone(),
        authority: ctx.accounts.user.to_account_info(),
        function: ctx.accounts.function.to_account_info(),
        function_authority: None,
        escrow: ctx.accounts.request_escrow.clone(),
        mint: ctx.accounts.mint.to_account_info(),
        state: ctx.accounts.state.to_account_info(),
        attestation_queue: ctx.accounts.attestation_queue.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
    };
    request_init_ctx.invoke(
        ctx.accounts.switchboard.clone(),
        None,
        Some(1000),
        Some(512),
        Some(request_params.into_bytes()),
        None,
        None,
    )?;

    let mut user = ctx.accounts.user.load_mut()?;
    user.last_round = user.current_round;
    user.current_round = UserRound {
        guess,
        wager,
        request: ctx.accounts.request.key(),
        status: RoundStatus::Pending,
        result: 0,
        slot: Clock::get()?.slot,
        timestamp: Clock::get()?.unix_timestamp,
    };

    Ok(())
}