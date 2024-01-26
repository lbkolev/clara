use jsonrpsee::http_client::HttpClientBuilder;

use clara::rpc::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClientBuilder::default().build("https://mainnet.era.zksync.io")?;
    let server = Server::new(client);

    match server.run().await {
        Ok((addr, handle)) => {
            println!("port {}", addr);
            handle.stopped().await;
        }
        Err(e) => {
            println!("Failed to start server: {}", e);
        }
    }

    Ok(())
}
