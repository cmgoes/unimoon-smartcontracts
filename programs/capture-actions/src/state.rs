use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub score: u128,
}

#[account]
pub struct Post {
    pub views: u8,
    pub likes: u8,
    pub shares: u8,
    pub total_comments: u8,
    pub downloads: u8,
    pub creator: Pubkey,
}
