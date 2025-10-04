use std::error::Error;

mod client;
mod wire;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    client::run().await
}
