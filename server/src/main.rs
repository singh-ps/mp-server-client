use std::error::Error;

mod server;
mod wire;

use server::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await
}
