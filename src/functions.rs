use postgrest::Postgrest;
use dotenvy::dotenv;
use reqwest;
use std::time::Instant;
use crate::structs::{Auctionhouse, Binauctions, Client};

// default initialization
impl Default for Client {
    fn default() -> Client {

        dotenv().ok();

        let supabase_endpoint = dotenvy::var("SUPABASE_ENDPOINT").unwrap();
        let supabase_anon_key = dotenvy::var("SUPABASE_ANON_KEY").unwrap();

        let supabase = Postgrest::new(&supabase_endpoint)
            .insert_header("apikey", &supabase_anon_key)
            .insert_header("Authorization", format!("Bearer {}", &supabase_anon_key));

        Client {
            supabase: supabase,
        }
    }
}

impl Client {
    pub async fn clear (&self, table_name: &str) {

        let response = self.supabase
            .from(table_name)
            .delete()
            .neq("uuid", "null")
            .execute()
            .await;

        match response {
            Ok(_) => println!("Cleared table: {}", table_name),
            Err(e) => println!("Error clearing table: {}", e),
        }
    }

    pub async fn upload_data (&self, table_name: &str, data: String) {

        let response = self.supabase
            .from(table_name)
            .insert(data)
            .execute()
            .await;

        match response {
            Ok(_) => println!("Uploaded data to table: {}", table_name),
            Err(e) => println!("Error uploading data: {}", e),
        }
    }
}


pub async fn get_data() -> Result<std::string::String, Box<dyn std::error::Error>> {

    let start_time = Instant::now();
    let client = reqwest::Client::new();
    let mut all_auctions: Vec<Binauctions> = Vec::new();
    let url = "https://api.hypixel.net/v2/skyblock/auctions";

    let response: Auctionhouse = client.get(url).send().await?.json().await?;

    for page in 0..response.totalPages {
        let url = format!("https://api.hypixel.net/v2/skyblock/auctions?page={}", page);
        let response: Auctionhouse = client.get(&url).send().await?.json().await?;

        for item in response.auctions.into_iter().filter(|i| i.bin) {
            all_auctions.push(Binauctions {
                uuid: item.uuid,
                item_name: item.item_name,
                item_lore: item.item_lore,
                price: item.starting_bid,
            });
        }
    }

    println!("Data fetched in: {:?}", start_time.elapsed());

    Ok(
        serde_json::to_string(&all_auctions).unwrap()
    )
}

