pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EeJkegNic33ejuVs6msY2QAGYx5DUCXnfk2PDaHyuz1D");

#[program]
pub mod tender_tracker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }
    pub fn create_tender(ctx: Context<CreateTender>, tender_id: u64, title: String, description: String,
        price_weight: u8, timeline_weight: u8, deadline: i64)->Result<()>{
        crate::instructions::create_tender::handle_create_tender(ctx, tender_id, title, description, price_weight, timeline_weight, deadline)
    }
    
    pub fn submit_bid(ctx: Context<SubmitBid>, price: u64, timeline_days: u32, quality_cert: String)->Result<()>{
        crate::instructions::submit_bid::handle_submit_bid(ctx, price, timeline_days, quality_cert)
    }

}
