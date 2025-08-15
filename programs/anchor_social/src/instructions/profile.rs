use crate::state::profile::*;
use anchor_lang::prelude::*;

pub fn create_profile(ctx: Context<CreateProfile>, display_name: String) -> Result<()> {
    ctx.accounts.profile.display_name = display_name;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = user, space = 8 + Profile::INIT_SPACE, seeds = [Profile::SEED_PREFIX.as_bytes(), user.key().as_ref()], bump)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
