use anchor_lang::prelude::*;
use crate::state::{Tender, Bid, Custom};
 use crate::error::{ErrorCode};
#[derive(Accounts)]

pub struct FinalizeTender<'info>{
#[account(mut)]
pub tender: Account<'info, Tender>,
#[account(
    mut,
    constraint = authority.key() == tender.authority @ ErrorCode::Unauthorized
)]
pub authority: Signer<'info>,
}
pub fn handle_finalize_tender(ctx: Context<FinalizeTender>) -> Result<()> {
    let mut min_price: u64 = u64::MAX;
    let mut min_timeline: u32 = u32::MAX;

    for account in ctx.remaining_accounts.iter() {
        let bid_data = Bid::try_deserialize(&mut &account.data.borrow()[..])?;
        if bid_data.price < min_price {
            min_price = bid_data.price;
        }
        if bid_data.timeline_days < min_timeline {
            min_timeline = bid_data.timeline_days;

    }
    }
    let mut best_score: f64 = -1.0;
    let mut winner_key: Option<Pubkey> = None;

    for account in ctx.remaining_accounts.iter() {
        let bid_data = Bid::try_deserialize(&mut &account.data.borrow()[..])?;
        let price_ratio = min_price as f64 / bid_data.price as f64 ;
        let timeline_ratio = min_timeline as f32 / bid_data.timeline_days as f32 ;
        
        let total_weight = (ctx.accounts.tender.price_weight as f64) + (ctx.accounts.tender.timeline_weight as f64);
        let score = (price_ratio * ctx.accounts.tender.price_weight as f64 
           + timeline_ratio as f64 * ctx.accounts.tender.timeline_weight as f64) / total_weight;
        if score > best_score {
            best_score = score;
            winner_key = Some(account.key());
        }
    }

    ctx.accounts.tender.winner = winner_key;
    ctx.accounts.tender.status = Custom::Finalized;
    Ok(())
}
