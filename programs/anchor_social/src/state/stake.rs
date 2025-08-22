use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct StakeInfo {
    pub staker: Pubkey,           // 质押人
    pub nft_mint_account: Pubkey, // 质押的nft
    pub staked_at: u64,           // 质押的时间
}

impl StakeInfo {
    pub const SEED_PREFIX: &'static str = "stake_v1";

    pub fn new(staker: Pubkey, nft_mint_account: Pubkey) -> Self {
        Self {
            staker,
            nft_mint_account,
            staked_at: Clock::get().unwrap().epoch,
        }
    }
}
