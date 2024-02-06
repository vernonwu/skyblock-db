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
            data: String::new(),
        }
    }
}

#[allow(non_snake_case)]
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

    pub async fn fetch_hypixel_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let start_time = Instant::now();
        let APIclient = reqwest::Client::new();
        let url = "https://api.hypixel.net/v2/skyblock/auctions";
        let response: Auctionhouse = APIclient.get(url).send().await?.json().await?;

        println!("Fetching data from Hypixel API...");
        let mut tasks = Vec::new();

        for page in 0..response.totalPages {
            let url = format!("https://api.hypixel.net/v2/skyblock/auctions?page={}", page);
            let client = APIclient.clone(); // Clone the client for each task

            let task: tokio::task::JoinHandle<Result<Vec<Binauctions>, reqwest::Error>> = tokio::spawn(async move {
                let response: Auctionhouse = client.get(&url).send().await?.json().await?;

                let mut auctions = Vec::new();
                for item in response.auctions.into_iter().filter(|i| i.bin) {
                    auctions.push(Binauctions {
                        uuid: item.uuid,
                        item_name: item.item_name,
                        item_lore: item.item_lore,
                        price: item.starting_bid,
                    });
                }

                Ok(auctions)
            });

            tasks.push(task);
        }

        let mut all_auctions = Vec::new();
        for task in tasks {
            let auctions: Result<Vec<Binauctions>, _> = task.await?;
            all_auctions.extend(auctions?);
        }

        println!("Data fetched in: {:?}", start_time.elapsed());

        let data = serde_json::to_string(&all_auctions).unwrap();
        self.data = data;

        Ok(())
    }

    pub async fn upload_data (&self, table_name: &str) {

        let start_time = Instant::now();
        println!("Uploading data to table: {}", table_name);

        let response = self.supabase
            .from(table_name)
            .insert(&self.data)
            .execute()
            .await;

        match response {
            Ok(_) => println!("Uploaded data to table: {} in {}s", table_name, start_time.elapsed().as_secs_f32()),
            Err(e) => println!("Error uploading data: {}", e),
        }
    }
}