use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Tweet {
    #[max_len(50)]
    pub body: String,

    pub like_count: u64,
}
impl Tweet {
    pub const SEED_PREFIX: &'static str = "tweet";

    pub fn new(body: String) -> Self {
        Self {
            body,
            like_count: 0,
        }
    }
}
