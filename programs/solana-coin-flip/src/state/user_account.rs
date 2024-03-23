use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub authority: Pubkey, // 32
    pub total_games_played: u64, // 8
    pub total_wins: u64, // 8
    pub total_losses: u64, // 8
    pub time: i64, // 8
    pub bump: u8, // 1
}

impl UserAccount {
    pub const SPACE: usize = 32 + 8 + 8 + 8 + 8 + 8 + 1;
}