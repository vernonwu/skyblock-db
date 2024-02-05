mod structs;
mod client;

use crate::client::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // init client
    let client = structs::Client::default();

    let table_name = "AUCTIONS";
    let resp = client.clear(table_name).await;
    
    Ok(())
}
