use anchor_lang::prelude::*;


#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum Custom {
    Open,
    Finalized
}
#[account]
#[derive(InitSpace)]
pub struct Tender {
    pub tender_id: u64,
    pub authority: Pubkey,
    #[max_len(100)]
    pub title: String,
    #[max_len(300)]
    pub description: String,
    pub price_weight: u8,
    pub timeline_weight: u8,
    pub deadline: i64,
    pub status: Custom,
    pub winner: Option<Pubkey>,

}

#[account]
#[derive(InitSpace)]
pub struct Bid{
    pub tender: Pubkey,
    pub bidder: Pubkey,
    pub price: u64,
    pub timeline_days: u32,
    #[max_len(300)]
    pub quality_cert: String,
    pub score: Option<u64>,



}
