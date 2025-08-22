use std::cmp::max;

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

    // 计算的利息收入
    pub fn salvage_value(&self, amount: u64) -> u64 {
        let now = Clock::get().unwrap().epoch;
        // 每一个epoch减2%
        // 下面代码有错 会返回0
        // let epoch = now.checked_sub(self.staked_at).unwrap_or(0);
        // let p = max(0, epoch.checked_mul(2)?).checked_div(100)?;

        // amount.checked_mul(p)

        let p = max(0, (now - self.staked_at) * 2) as f64 / 100.0;
        (amount as f64 * p) as u64
    }
}
