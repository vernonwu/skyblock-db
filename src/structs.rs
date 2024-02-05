use serde::{Deserialize, Serialize};
use postgrest::Postgrest;

#[derive(Debug, Deserialize)]
pub struct Auctionhouse{
    pub total_pages: u8,
    pub auctions: Vec<Auctionitem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auctionitem {
    pub uuid: String,
    pub item_name: String,
    pub item_lore: String,
    pub starting_bid: u64,
    pub bin: bool,
}

pub struct Client {
    pub supabase: Postgrest,
}
