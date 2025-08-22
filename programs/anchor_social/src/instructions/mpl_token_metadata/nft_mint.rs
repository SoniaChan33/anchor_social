use crate::state::nft::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::create_master_edition_v3;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::mpl_token_metadata::instructions::CreateV1CpiBuilder;
use anchor_spl::metadata::mpl_token_metadata::instructions::MintV1CpiBuilder;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::mpl_token_metadata::types::PrintSupply;
use anchor_spl::metadata::CreateMasterEditionV3;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::mint_to;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
// 方法一
pub fn nft_mint_v1(ctx: Context<NFTMint>, nft_id: String) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[
        MyNFT::SEED_PREFIX.as_bytes(),
        nft_id.as_bytes(),
        &[ctx.bumps.nft_mint_account],
    ]];
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.meta_account.to_account_info(),
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
                update_authority: ctx.accounts.nft_mint_account.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        DataV2 {
            name: format!("{} #{}", MyNFT::TOKEN_NAME.to_string(), nft_id),
            symbol: MyNFT::TOKEN_SYMBOL.to_string(),
            uri: MyNFT::TOKEN_URL.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false,
        true,
        None,
    )?;

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                to: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.nft_mint_account.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    // 创建 NFT 的主版本账户，通过限制最大发行量（这里是 1）来确保该 NFT 无法被增发，从而保证其唯一性
    create_master_edition_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                metadata: ctx.accounts.meta_account.to_account_info(),
                mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
                update_authority: ctx.accounts.nft_mint_account.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        Some(1), // 最大发行量
    )?;

    Ok(())
}

// 方法二 使用metadata的函数
// pub fn nft_mint_v2(ctx: Context<NFTMint>, nft_id: String) -> Result<()> {
//     let signer_seeds: &[&[&[u8]]] = &[&[
//         MyNFT::SEED_PREFIX.as_bytes(),
//         nft_id.as_bytes(),
//         &[ctx.bumps.nft_mint_account],
//     ]];

//     CreateV1CpiBuilder::new(ctx.accounts.token_metadata_program.as_ref())
//         .mint(ctx.accounts.nft_mint_account.as_ref(), true)
//         .metadata(ctx.accounts.meta_account.as_ref())
//         .master_edition(Some(ctx.accounts.master_edition_account.as_ref()))
//         .authority(ctx.accounts.nft_mint_account.as_ref())
//         .update_authority(ctx.accounts.nft_mint_account.as_ref(), true)
//         .payer(ctx.accounts.authority.as_ref())
//         .seller_fee_basis_points(0)
//         .system_program(ctx.accounts.system_program.as_ref())
//         .sysvar_instructions(ctx.accounts.rent.as_ref())
//         .spl_token_program(Some(ctx.accounts.token_program.as_ref()))
//         .name(MyNFT::TOKEN_NAME.to_string())
//         .symbol(MyNFT::TOKEN_SYMBOL.to_string())
//         .uri(MyNFT::TOKEN_URL.to_string())
//         .print_supply(PrintSupply::Zero)
//         .invoke_signed(signer_seeds)?;

//     MintV1CpiBuilder::new(ctx.accounts.token_metadata_program.as_ref())
//         .mint(ctx.accounts.nft_mint_account.as_ref())
//         .metadata(ctx.accounts.meta_account.as_ref())
//         .master_edition(Some(ctx.accounts.master_edition_account.as_ref()))
//         .authority(ctx.accounts.nft_mint_account.as_ref())
//         .payer(ctx.accounts.authority.as_ref())
//         // .seller_fee_basis_points(0)
//         .system_program(ctx.accounts.system_program.as_ref())
//         .sysvar_instructions(ctx.accounts.rent.as_ref())
//         .spl_token_program(Some(ctx.accounts.token_program.as_ref()))
//         .name(MyNFT::TOKEN_NAME.to_string())
//         .symbol(MyNFT::TOKEN_SYMBOL.to_string())
//         .uri(MyNFT::TOKEN_URL.to_string())
//         .print_supply(PrintSupply::Zero)
//         .invoke_signed(signer_seeds)?;

//     Ok(())
// }

#[derive(Accounts)]
#[instruction(nft_id: String)] // 这里作用是将nft_id作为参数传递给指令
pub struct NFTMint<'info> {
    /// CHECK : This is the metadata account for the token
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint_account.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub master_edition_account: UncheckedAccount<'info>,

    /// CHECK : This is the metadata account for the token
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint_account.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub meta_account: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [MyNFT::SEED_PREFIX.as_bytes(), &nft_id.to_string().as_bytes()],
        bump,
        mint::decimals = MyNFT::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(), // 谁有权限去mint
        mint::freeze_authority = nft_mint_account.key(),
    )]
    pub nft_mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = nft_mint_account,
        associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}
