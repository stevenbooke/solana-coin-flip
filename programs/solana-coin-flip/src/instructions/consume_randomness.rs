use anchor_lang::prelude::*;
use solana_randomness_service::SimpleRandomnessV1Account;
use solana_randomness_service::ID as SolanaRandomnessServiceID;
use switchboard_solana::prelude::*;

#[derive(Accounts)]
pub struct ConsumeRandomness<'info> {
    /// We need to make sure the randomness service signed this requests so it can only be invoked by a PDA and not a user.
    #[account(
        signer,
        seeds = [b"STATE"],
        seeds::program = SolanaRandomnessServiceID,
        bump = randomness_state.bump,
    )]
    pub randomness_state: Box<Account<'info, solana_randomness_service::State>>,

    pub request: Box<Account<'info, SimpleRandomnessV1Account>>,
}

pub fn consume_randomness(
    _ctx: Context<ConsumeRandomness>,
    result: Vec<u8>,
) -> anchor_lang::prelude::Result<()> {
    msg!("Randomness received: {:?}", result);
    Ok(())
}

