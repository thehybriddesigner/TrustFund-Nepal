use anchor_lang::prelude::*;
use crate::state::{Tender, Custom};

#[derive(Accounts)]
#[instruction(tender_id: u64)]
pub struct CreateTender<'info>{
#[account(mut)]
pub payer: Signer<'info>,
#[account(
init,
payer=payer,
space = 8 + Tender::INIT_SPACE,
seeds = [b"tender", tender_id.to_le_bytes().as_ref()],
bump
)]
pub tender: Account<'info, Tender>,
pub system_program: Program<'info, System>,

}

pub fn handle_create_tender(ctx: Context<CreateTender>,tender_id: u64, title: String, description: String, price_weight: u8, timeline_weight: u8, deadline: i64)->Result<()>{
    ctx.accounts.tender.authority = ctx.accounts.payer.key();
    ctx.accounts.tender.tender_id = tender_id;
    ctx.accounts.tender.title = title;
    ctx.accounts.tender.description = description;
    ctx.accounts.tender.price_weight = price_weight;
    ctx.accounts.tender.timeline_weight = timeline_weight;
    ctx.accounts.tender.deadline  = deadline;
    ctx.accounts.tender.status = Custom::Open;
    ctx.accounts.tender.winner = None;
    Ok(())
}



