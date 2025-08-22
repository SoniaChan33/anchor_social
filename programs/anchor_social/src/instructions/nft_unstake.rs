use crate::state::nft::MyNFT;
use crate::state::stake::StakeInfo;
use crate::state::token::MyToken;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::burn;
use anchor_spl::token::mint_to;
use anchor_spl::token::transfer;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

#[error_code]
pub enum UnstakeError {
    #[msg("can not unstake")]
    NoAuthority,
}

pub fn nft_unstake(ctx: Context<NFTUnStake>) -> Result<()> {
    // 检查质押关系
    require!(
        &ctx.accounts.stake_info.nft_mint_account == &ctx.accounts.nft_mint_account.key(),
        UnstakeError::NoAuthority,
    );
    require!(
        &ctx.accounts.stake_info.staker == &ctx.accounts.authority.key(),
        UnstakeError::NoAuthority,
    );

    // 这里就需要反向transfer一下了
    //todo 为什么反向的就需要seeds
    let nft_mint_account = ctx.accounts.nft_mint_account.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        StakeInfo::SEED_PREFIX.as_bytes(),
        nft_mint_account.as_ref(),
        &[ctx.bumps.stake_info],
    ]];

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.program_receipt_ata.to_account_info(),
                to: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.stake_info.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        1,
    )?;

    // 销毁流动性代币
    let amount = ctx.accounts.stake_info.salvage_value(10000);
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint_account.to_account_info(),
                from: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct NFTUnStake<'info> {
    #[account(
        mut,
        seeds = [
            StakeInfo::SEED_PREFIX.as_bytes(),
            nft_mint_account.key().as_ref(),
        ],
        bump,
    )]
    pub stake_info: Box<Account<'info, StakeInfo>>,

    // 接收NFT的token账户
    #[account(
        mut,
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
        mut,
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
