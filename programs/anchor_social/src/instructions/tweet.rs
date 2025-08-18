use crate::state::like::*;
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
        space = 8 + Tweet::INIT_SPACE,
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

pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    tweet.like_count += 1;

    let like = Like::new(ctx.accounts.authority.key(), tweet.key());
    // TODO 这里的setinner是什么
    ctx.accounts.like.set_inner(like);
    Ok(())
}

#[derive(Accounts)]
pub struct CreateLike<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Like::INIT_SPACE,
        seeds = [
            Like::SEED_PREFIX.as_bytes().as_ref(),
            profile.key().as_ref(),
            tweet.key().as_ref()
        ],
        bump
    )]
    pub like: Account<'info, Like>,

    #[account(mut)]
    pub tweet: Account<'info, Tweet>,

    // TODO 为什么每次指令里面的profile都需要也有一个种子参数？
    #[account(mut, seeds = [Profile::SEED_PREFIX.as_bytes(), authority.key().as_ref()], bump)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
