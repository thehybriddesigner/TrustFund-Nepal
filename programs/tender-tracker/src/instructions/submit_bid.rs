use anchor_lang::prelude::*;
use crate::state::{Tender, Bid};
use crate::error::{ErrorCode};

#[derive(Accounts)]
#[instruction(tender_id: u64)]
pub struct SubmitBid<'info>{
#[account(mut)]
pub bidder: Signer<'info>,
#[account(
init,
payer=bidder,
space = 8 + Bid::INIT_SPACE,
seeds = [b"bid",tender_id.to_le_bytes().as_ref(), bidder.key().as_ref()],
bump
)]
pub bid: Account<'info, Bid>,
#[account(
    constraint = Clock::get()?.unix_timestamp < tender.deadline @ ErrorCode::DeadlinePassed
)]
pub tender: Account<'info, Tender>,
pub system_program: Program<'info, System>,

}

pub fn handle_submit_bid(ctx: Context<SubmitBid>,price: u64, timeline_days: u32, quality_cert: String )->Result<()>{
    ctx.accounts.bid.tender= ctx.accounts.tender.key();
    ctx.accounts.bid.bidder = ctx.accounts.bidder.key();
    ctx.accounts.bid.price = price;
    ctx.accounts.bid.timeline_days = timeline_days;
    ctx.accounts.bid.quality_cert  = quality_cert;
ctx.accounts.bid.score = None;
	Ok(())
}
