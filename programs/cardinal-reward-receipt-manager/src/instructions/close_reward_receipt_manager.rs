use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct CloseRewardReceiptManagerCtx<'info> {
    #[account(mut, close = authority)]
    reward_receipt_manager: Box<Account<'info, RewardReceiptManager>>,
    #[account(mut, constraint = reward_receipt_manager.authority == authority.key() @ ErrorCode::InvalidAuthority)]
    authority: Signer<'info>,
}

pub fn handler(_ctx: Context<CloseRewardReceiptManagerCtx>) -> Result<()> {
    Ok(())
}
