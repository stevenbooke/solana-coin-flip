use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    #[account(
        init,
        payer = authority,
        space = UserAccount::SPACE,
        seeds = [b"create-user-account".as_ref(), authority.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    let authority = &mut ctx.accounts.authority;

    user_account.authority = authority.key();
    user_account.total_games_played = 0;
    user_account.total_wins = 0;
    user_account.total_losses = 0;
    user_account.bump = ctx.bumps.user_account;
    user_account.time = Clock::get()?.unix_timestamp;

    Ok(())
}