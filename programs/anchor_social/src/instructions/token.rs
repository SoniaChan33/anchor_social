use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;

pub fn create_token_mint_account(ctx: Context<CreateTokenMintAccount>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTokenMintAccount<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        seeds = [b"mint",],
        bump,
        mint::decimals = 9,
        mint::authority = mint_account.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
