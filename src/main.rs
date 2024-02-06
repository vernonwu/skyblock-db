mod structs;
mod functions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // init client
    let mut client = structs::Client::default();
    let table_name = "AUCTIONS";

    // clear all data from table
    client.clear(table_name).await;

    client.fetch_hypixel_data().await?;

    client.upload_data(table_name).await;

    Ok(())
}
