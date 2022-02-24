use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub sac: u64,
}

#[account]
pub struct Post {
    pub views: u64,
    pub likes: u64,
    pub shares: u64,
    pub total_comments: u64,
    pub downloads: u64,
    pub creator: Pubkey,
    pub sac: u64,
    pub token: Pubkey,
}
