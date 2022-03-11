use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserSacPair {
    pub user: Pubkey,
    pub sac: u64,
}

#[account]
#[derive(Default)]
pub struct Unimoon {
    pub bump: u8,
    pub pairs: Vec<UserSacPair>,
}
