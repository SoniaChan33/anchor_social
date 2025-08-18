use crate::state::profile::*;
use crate::state::tweet::*;
use anchor_lang::prelude::*;

pub fn create_tweet(ctx: Context<CreateTweet>, body: String) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    profile.tweet_count += 1;
    let tweet = Tweet::new(body);
    ctx.accounts.tweet.set_inner(tweet.clone());
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTweet<'info> {
    #[account(
        init,
        payer = authority,
        space = Tweet::INIT_SPACE,
        seeds = [
            Tweet::SEED_PREFIX.as_bytes(),
            profile.key().as_ref(),
            (profile.tweet_count + 1).to_string().as_ref()
        ],
        bump
    )]
    pub tweet: Account<'info, Tweet>,

    #[account(mut, seeds = [Profile::SEED_PREFIX.as_bytes(), authority.key().as_ref()], bump)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
