use anchor_lang::error_code;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum VrfFlipError {
    #[msg("VRF Account counter should be 0 for a new lottery")]
    InvalidInitialVrfCounter,
    #[msg("VRF Account authority should be the lottery Pubkey")]
    InvalidVrfAuthority,
    #[msg("Provided account is not owned by the switchboard program")]
    InvalidSwitchboardAccount,
    #[msg("VRF counter does not match the expected round id")]
    IncorrectVrfCounter,
    #[msg("Failed to match the game type")]
    InvalidGameType,
    #[msg("Current round is still active")]
    CurrentRoundStillActive,
    #[msg("Current round has already settled")]
    CurrentRoundAlreadyClosed,
    #[msg("Invalid bet")]
    InvalidBet,
    #[msg("Switchboard queue requires VRF permissions to request randomness")]
    OracleQueueRequiresPermissions,
    #[msg("VRF account belongs to the incorrect oracle queue")]
    OracleQueueMismatch,
    #[msg("User requested an airdrop too soon")]
    AirdropRequestedTooSoon,
    #[msg("User has enough funds and does not require an airdrop")]
    UserTokenBalanceHealthy,
    #[msg("Max bet exceeded")]
    MaxBetAmountExceeded,
    #[msg("Insufficient funds to request randomness")]
    InsufficientFunds,
    #[msg("User can flip once every 10 seconds")]
    FlipRequestedTooSoon,
    #[msg("House has no authority to mint more tokens")]
    UnauthorizedMint,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum RandomnessRequestError {
    #[msg("Invalid authority account")]
    InvalidAuthority,
    #[msg("Invalid escrow account")]
    InvalidEscrow,
    #[msg("Array overflow")]
    ArrayOverflow,
    #[msg("Stale data")]
    StaleData,
    #[msg("Invalid trusted signer")]
    InvalidTrustedSigner,
    #[msg("Invalid MRENCLAVE")]
    InvalidMrEnclave,
    #[msg("Failed to find a valid trading symbol for this price")]
    InvalidSymbol,
    #[msg("FunctionAccount pubkey did not match program_state.function")]
    IncorrectSwitchboardFunction,
    #[msg("FunctionAccount pubkey did not match program_state.function")]
    InvalidSwitchboardFunction,
    #[msg("FunctionAccount was not validated successfully")]
    FunctionValidationFailed,
    #[msg("FunctionRequestAccount status should be 'RequestSuccess'")]
    SwitchboardRequestNotSuccessful,
    #[msg("Round is inactive")]
    RoundInactive,
    #[msg("House has insufficient funds to payout winners")]
    HouseInsufficientFunds,
}