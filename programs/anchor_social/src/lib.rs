use anchor_lang::prelude::*;

declare_id!("35vQtxXXv5rb99eiVrVVrwYMRYc7vscZvXas8zjEnnK5");

pub mod instructions;
pub mod state;
use instructions::*;

#[program]
pub mod anchor_social {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, display_name: String) -> Result<()> {
        instructions::profile::create_profile(ctx, display_name)
    }

    pub fn create_tweet(ctx: Context<CreateTweet>, body: String) -> Result<()> {
        instructions::tweet::create_tweet(ctx, body)
    }
}
