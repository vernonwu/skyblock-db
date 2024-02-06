mod structs;
mod functions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // init client
    let client = structs::Client::default();
    let table_name = "AUCTIONS";

    // clear all data from table
    client.clear(table_name).await;

    let auctions = match functions::get_data().await {
        Ok(auctions) => auctions,
        Err(e) => panic!("Error getting data: {}", e),
    };

    // upload data to table
    client.upload_data(table_name, auctions).await;

    Ok(())
}
