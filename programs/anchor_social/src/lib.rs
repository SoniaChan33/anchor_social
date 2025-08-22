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

    pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
        instructions::tweet::create_like(ctx)
    }

    pub fn create_token_mint_account(ctx: Context<CreateTokenMintAccount>) -> Result<()> {
        instructions::token::create_token_mint_account(ctx)
    }

    pub fn nft_mint(ctx: Context<NFTMint>, nft_id: String) -> Result<()> {
        instructions::mpl_token_metadata::nft_mint_v1(ctx, nft_id)
    }

    pub fn nft_stake(ctx: Context<NFTStake>) -> Result<()> {
        instructions::nft_stake::nft_stake(ctx)
    }

    pub fn nft_unstake(ctx: Context<NFTUnStake>) -> Result<()> {
        instructions::nft_unstake::nft_unstake(ctx)
    }
}
