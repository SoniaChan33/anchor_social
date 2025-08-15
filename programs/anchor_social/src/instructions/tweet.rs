use crate::state::tweet::*;
use anchor_lang::prelude::*;

pub fn create_tweet(ctx: Context<CreateTweet>, body: String) -> Result<()> {
    ctx.accounts.tweet.body = body;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTweet<'info> {
    #[account(init, payer = authority, space = 8 + Tweet::INIT_SPACE, seeds = [Tweet::SEED_PREFIX.as_bytes(), authority.key().as_ref()], bump)]
    pub tweet: Account<'info, Tweet>,

    #[account(mut, seeds = [Profile::SEED_PREFIX.as_bytes(), authority.key().as_ref()], bump)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
