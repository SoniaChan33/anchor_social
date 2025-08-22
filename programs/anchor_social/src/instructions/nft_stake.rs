use crate::state::nft::MyNFT;
use crate::state::stake::StakeInfo;
use crate::state::token::MyToken;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::mint_to;
use anchor_spl::token::transfer;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

pub fn nft_stake(ctx: Context<NFTStake>) -> Result<()> {
    // 记录质押关系
    let stake_info = StakeInfo::new(
        ctx.accounts.authority.key(),
        ctx.accounts.nft_mint_account.key(),
    );
    ctx.accounts.stake_info.set_inner(stake_info.clone());

    // transfer the nft from the user's associated token account to the staking pool
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_associated_token_account.to_account_info(),
                to: ctx.accounts.program_receipt_ata.to_account_info(), // todo
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        1,
    )?;

    // mint 流动性代币
    let signer_seeds: &[&[&[u8]]] = &[&[
        MyToken::SEED_PREFIX.as_bytes(),
        &[ctx.bumps.token_mint_account],
    ]];

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.token_mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.token_mint_account.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        10000,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct NFTStake<'info> {
    /// 接收NFT的账户
    /// 用于存储质押信息
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + StakeInfo::INIT_SPACE,
        seeds = [
            StakeInfo::SEED_PREFIX.as_bytes(),
            nft_mint_account.key().as_ref(),
        ],
        bump,
    )]
    pub stake_info: Box<Account<'info, StakeInfo>>,

    // 接收NFT的token账户
    #[account(
        init_if_needed,
        payer = authority, // 注意这里的payer是自己账户
        associated_token::mint = nft_mint_account,
        associated_token::authority = stake_info,
    )]
    pub program_receipt_ata: Box<Account<'info, TokenAccount>>,

    /// 获取的流动性token
    /// 检查token是不是合约派生pda？
    #[account(
        mut,
        seeds = [
            MyToken::SEED_PREFIX.as_bytes(),
        ],
        bump,
    )]
    pub token_mint_account: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = token_mint_account,
        associated_token::authority = authority,
    )]
    pub associated_token_account: Box<Account<'info, TokenAccount>>,

    /// 质押的NFT
    #[account(mut)]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    #[account(mut,
        associated_token::mint = nft_mint_account,
        associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
