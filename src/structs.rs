use serde::{Deserialize, Serialize};
use postgrest::Postgrest;
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Auctionhouse{
    pub totalPages: u8,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Binauctions {
    pub uuid: String,
    pub item_name: String,
    pub item_lore: String,
    pub price: u64,
}
pub struct Client {
    pub supabase: Postgrest,
}
