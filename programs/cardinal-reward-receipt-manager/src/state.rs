use anchor_lang::prelude::*;

pub const REWARD_RECEIPT_MANAGER_SEED: &str = "reward-receipt-manager";
pub const REWARD_RECEIPT_MANAGER_SIZE: usize = 8 + std::mem::size_of::<RewardReceiptManager>() + 64;
#[account]
pub struct RewardReceiptManager {
    pub bump: u8,
    pub name: String,
    pub stake_pool: Pubkey,
    pub authority: Pubkey,
    pub receipts_counter: u128,
    pub max_reward_receipts: Option<u128>,
}

pub const REWARD_RECEIPT_SEED: &str = "reward-receipt";
pub const REWARD_RECEIPT_SIZE: usize = 8 + std::mem::size_of::<RewardReceipt>() + 64;
#[account]
pub struct RewardReceipt {
    pub bump: u8,
    pub reward_receipt_manager: Pubkey,
    pub target: Pubkey,
}
