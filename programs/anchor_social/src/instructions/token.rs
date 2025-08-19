use anchor_lang::prelude::*;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;

pub fn create_token_mint_account(ctx: Context<CreateTokenMintAccount>) -> Result<()> {
    // TODO 这个是啥 怎么进行定义的？
    let signer_seeds: &[&[&[u8]]] = &[&[b"mint_v3", &[ctx.bumps.mint_account]]];
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.meta_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                mint_authority: ctx.accounts.mint_account.to_account_info(),
                update_authority: ctx.accounts.mint_account.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        // TODO 这是token的内容吗
        DataV2 {
            name: "tokenchs".to_string(),
            symbol: "TKS".to_string(),
            uri: "https://example.com/tokenchs".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false,
        true,
        None,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTokenMintAccount<'info> {
    /// CHECK : This is the metadata account for the token
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint_account.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub meta_account: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [b"mint_v3",],
        bump,
        mint::decimals = 100,
        mint::authority = mint_account.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,

    pub rent: Sysvar<'info, Rent>,
}
