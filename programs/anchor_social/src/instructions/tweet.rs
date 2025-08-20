use crate::state::like::*;
use crate::state::profile::*;
use crate::state::tweet::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::mint_to;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

pub fn create_tweet(ctx: Context<CreateTweet>, body: String) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    profile.tweet_count += 1;
    let tweet = Tweet::new(body, ctx.accounts.tweet.key());
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

    let like = Like::new(ctx.accounts.profile.key(), tweet.key());
    ctx.accounts.like.set_inner(like);
    // 打印mint_account的地址
    msg!("mint_account: {}", ctx.accounts.mint_account.key());
    msg!(
        "author_token_account: {}",
        ctx.accounts.author_token_account.key()
    );

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.author_token_account.to_account_info(),
                authority: ctx.accounts.mint_account.to_account_info(),
            },
            &[&[b"mint_v3", &[ctx.bumps.mint_account]]],
        ),
        100,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateLike<'info> {
    #[account(
        mut,
        seeds = [b"mint_v3",],
        bump,
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint_account,
        associated_token::authority = author_wallet,
    )]
    pub author_token_account: Account<'info, TokenAccount>,

    /// CHECK : THIS IS AUTHOR WALLET
    pub author_wallet: AccountInfo<'info>,

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

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>, // 新增的Token程序字段
}
