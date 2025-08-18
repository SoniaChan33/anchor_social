use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Like {
    pub profile_pubkey: Pubkey,
    pub tweet_pubkey: Pubkey,
}
impl Like {
    pub const SEED_PREFIX: &'static str = "like";

    pub fn new(profile_pubkey: Pubkey, tweet_pubkey: Pubkey) -> Self {
        Self {
            profile_pubkey,
            tweet_pubkey,
        }
    }
}
